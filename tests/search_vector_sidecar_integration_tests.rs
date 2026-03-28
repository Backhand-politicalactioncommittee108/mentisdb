use mentisdb::search::{EmbeddingInput, EmbeddingMetadata, EmbeddingProvider, EmbeddingVector};
use mentisdb::{MentisDb, ThoughtQuery, ThoughtType, VectorSearchQuery};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use tempfile::TempDir;

#[derive(Debug, Clone)]
struct TestProviderError(String);

impl fmt::Display for TestProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for TestProviderError {}

#[derive(Clone)]
struct TestSemanticProvider {
    metadata: EmbeddingMetadata,
}

impl TestSemanticProvider {
    fn new(model_id: &str, embedding_version: &str) -> Self {
        Self {
            metadata: EmbeddingMetadata::new(model_id, 2, embedding_version),
        }
    }

    fn vector_for_text(&self, text: &str) -> Vec<f32> {
        let normalized = text.to_ascii_lowercase();
        if normalized.contains("latency")
            || normalized.contains("performance")
            || normalized.contains("budget")
        {
            vec![1.0, 0.0]
        } else if normalized.contains("invoice")
            || normalized.contains("vendor")
            || normalized.contains("payment")
        {
            vec![0.0, 1.0]
        } else {
            vec![0.2, 0.2]
        }
    }
}

impl EmbeddingProvider for TestSemanticProvider {
    type Error = TestProviderError;

    fn metadata(&self) -> &EmbeddingMetadata {
        &self.metadata
    }

    fn embed_batch(&self, inputs: &[EmbeddingInput]) -> Result<Vec<EmbeddingVector>, Self::Error> {
        Ok(inputs
            .iter()
            .map(|input| EmbeddingVector::new(self.vector_for_text(&input.text)))
            .collect())
    }
}

fn build_chain() -> (TempDir, MentisDb) {
    let tempdir = TempDir::new().unwrap();
    let chain_dir = PathBuf::from(tempdir.path());
    let mut chain = MentisDb::open_with_key(&chain_dir, "semantic-search").unwrap();
    chain
        .append(
            "planner",
            ThoughtType::Decision,
            "Latency budget for the Europe rollout.",
        )
        .unwrap();
    chain
        .append(
            "accounting",
            ThoughtType::Insight,
            "Invoice reconciliation for vendor payments.",
        )
        .unwrap();
    (tempdir, chain)
}

#[test]
fn rebuild_and_query_vector_sidecar_returns_semantic_hits() {
    let (_tempdir, chain) = build_chain();
    let provider = TestSemanticProvider::new("local-test", "v1");

    let sidecar = chain.rebuild_vector_sidecar(&provider).unwrap();
    let path = chain.vector_sidecar_path(provider.metadata()).unwrap();
    assert!(path.exists());
    assert_eq!(sidecar.metadata.model_id, "local-test");

    let result = chain
        .query_vector(
            &provider,
            &VectorSearchQuery::new("performance budget").with_limit(2),
        )
        .unwrap();

    assert_eq!(result.metadata.embedding_version, "v1");
    assert_eq!(result.total_candidates, 2);
    assert_eq!(result.hits.len(), 2);
    assert_eq!(
        result.hits[0].thought.content,
        "Latency budget for the Europe rollout."
    );
}

#[test]
fn vector_sidecar_status_turns_stale_after_append() {
    let (_tempdir, mut chain) = build_chain();
    let provider = TestSemanticProvider::new("local-test", "v1");
    let sidecar = chain.rebuild_vector_sidecar(&provider).unwrap();

    chain
        .append(
            "planner",
            ThoughtType::Idea,
            "Tail-latency mitigation for the next release.",
        )
        .unwrap();

    let result = chain
        .query_vector(
            &provider,
            &VectorSearchQuery::new("performance budget").with_limit(2),
        )
        .unwrap();

    assert!(matches!(
        result.freshness,
        mentisdb::search::VectorSidecarFreshness::StaleThoughtCount { .. }
            | mentisdb::search::VectorSidecarFreshness::StaleHeadHash { .. }
    ));
    assert_eq!(sidecar.entries.len(), 2);
    assert!(!result.hits.is_empty());
}

#[test]
fn vector_sidecar_paths_separate_model_versions() {
    let (_tempdir, chain) = build_chain();
    let provider_v1 = TestSemanticProvider::new("local-test", "v1");
    let provider_v2 = TestSemanticProvider::new("local-test", "v2");

    chain.rebuild_vector_sidecar(&provider_v1).unwrap();
    chain.rebuild_vector_sidecar(&provider_v2).unwrap();

    let path_v1 = chain.vector_sidecar_path(provider_v1.metadata()).unwrap();
    let path_v2 = chain.vector_sidecar_path(provider_v2.metadata()).unwrap();
    assert!(path_v1.exists());
    assert!(path_v2.exists());
    assert_ne!(path_v1, path_v2);
}

#[test]
fn corruption_does_not_break_plain_chain_queries() {
    let (_tempdir, chain) = build_chain();
    let provider = TestSemanticProvider::new("local-test", "v1");
    chain.rebuild_vector_sidecar(&provider).unwrap();
    let sidecar_path = chain.vector_sidecar_path(provider.metadata()).unwrap();
    let corrupted = std::fs::read_to_string(&sidecar_path)
        .unwrap()
        .replace("\"digest_hex\":\"", "\"digest_hex\":\"corrupted-");
    std::fs::write(&sidecar_path, corrupted).unwrap();

    let error = chain
        .query_vector(
            &provider,
            &VectorSearchQuery::new("performance budget").with_limit(2),
        )
        .unwrap_err();
    assert!(error.to_string().contains("integrity"));

    let plain = chain.query(&ThoughtQuery::new().with_text("invoice"));
    assert_eq!(plain.len(), 1);
    assert_eq!(
        plain[0].content,
        "Invoice reconciliation for vendor payments."
    );
}
