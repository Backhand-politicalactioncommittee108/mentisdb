use chrono::Utc;
use mentisdb::search::{
    EmbeddingMetadata, VectorSidecar, VectorSidecarEntry, VectorSidecarFreshness,
};
use std::fs;
use tempfile::TempDir;
use uuid::Uuid;

#[test]
fn vector_sidecar_round_trips_with_integrity() {
    let tempdir = TempDir::new().unwrap();
    let path = tempdir.path().join("chain.vectors.model.v1.json");
    let sidecar = VectorSidecar::build(
        "mentisdb",
        EmbeddingMetadata::new("local-model", 2, "v1"),
        2,
        Some("head-a".to_string()),
        Utc::now(),
        vec![
            VectorSidecarEntry::new(Uuid::new_v4(), 0, "hash-a", vec![1.0, 0.0]),
            VectorSidecarEntry::new(Uuid::new_v4(), 1, "hash-b", vec![0.0, 1.0]),
        ],
    )
    .unwrap();

    sidecar.save_to_path(&path).unwrap();
    let loaded = VectorSidecar::load_from_path(&path).unwrap();

    assert_eq!(loaded.chain_key, "mentisdb");
    assert_eq!(loaded.metadata.model_id, "local-model");
    assert_eq!(loaded.metadata.embedding_version, "v1");
    assert_eq!(loaded.entries.len(), 2);
    assert_eq!(loaded.integrity.entry_count, 2);
}

#[test]
fn vector_sidecar_detects_corruption() {
    let tempdir = TempDir::new().unwrap();
    let path = tempdir.path().join("chain.vectors.model.v1.json");
    let sidecar = VectorSidecar::build(
        "mentisdb",
        EmbeddingMetadata::new("local-model", 2, "v1"),
        1,
        Some("head-a".to_string()),
        Utc::now(),
        vec![VectorSidecarEntry::new(
            Uuid::new_v4(),
            0,
            "hash-a",
            vec![1.0, 0.0],
        )],
    )
    .unwrap();

    sidecar.save_to_path(&path).unwrap();
    let mut corrupted = fs::read_to_string(&path).unwrap();
    corrupted = corrupted.replace("\"hash-a\"", "\"hash-b\"");
    fs::write(&path, corrupted).unwrap();

    let error = VectorSidecar::load_from_path(&path).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("integrity"));
}

#[test]
fn vector_sidecar_freshness_detects_model_and_chain_drift() {
    let sidecar = VectorSidecar::build(
        "mentisdb",
        EmbeddingMetadata::new("local-model", 2, "v1"),
        2,
        Some("head-a".to_string()),
        Utc::now(),
        vec![
            VectorSidecarEntry::new(Uuid::new_v4(), 0, "hash-a", vec![1.0, 0.0]),
            VectorSidecarEntry::new(Uuid::new_v4(), 1, "hash-b", vec![0.0, 1.0]),
        ],
    )
    .unwrap();

    assert_eq!(
        sidecar.freshness(
            "mentisdb",
            2,
            Some("head-a"),
            &EmbeddingMetadata::new("local-model", 2, "v1"),
        ),
        VectorSidecarFreshness::Fresh
    );
    assert_eq!(
        sidecar.freshness(
            "mentisdb",
            2,
            Some("head-a"),
            &EmbeddingMetadata::new("local-model", 2, "v2"),
        ),
        VectorSidecarFreshness::EmbeddingVersionMismatch {
            expected: "v2".to_string(),
            actual: "v1".to_string(),
        }
    );
    assert_eq!(
        sidecar.freshness(
            "mentisdb",
            3,
            Some("head-b"),
            &EmbeddingMetadata::new("local-model", 2, "v1"),
        ),
        VectorSidecarFreshness::StaleThoughtCount {
            expected: 3,
            actual: 2,
        }
    );
}
