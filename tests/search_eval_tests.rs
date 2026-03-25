use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use mentisdb::{MentisDb, ThoughtInput, ThoughtQuery, ThoughtType};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_chain_dir() -> PathBuf {
    let n = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "mentisdb_search_eval_test_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn text_search_matches_registry_metadata_as_well_as_thought_fields() {
    let dir = unique_chain_dir();
    let mut chain = MentisDb::open_with_key(&dir, "search-registry-eval").unwrap();

    chain
        .upsert_agent(
            "search-ops",
            Some("Search Operations"),
            Some("retrieval-team"),
            Some("Owns lexical retrieval diagnostics."),
            None,
        )
        .unwrap();
    chain.add_agent_alias("search-ops", "navigator").unwrap();

    chain
        .append_thought(
            "search-ops",
            ThoughtInput::new(
                ThoughtType::Insight,
                "Keep lexical search explainable and deterministic.",
            )
            .with_tags(["search", "latency"])
            .with_concepts(["retrieval", "debugging"]),
        )
        .unwrap();

    for needle in [
        "search operations",
        "retrieval-team",
        "navigator",
        "diagnostics",
        "search-ops",
        "latency",
        "debugg",
        "deterministic",
    ] {
        let results = chain.query(&ThoughtQuery::new().with_text(needle));
        assert_eq!(results.len(), 1, "expected one hit for needle {needle}");
        assert_eq!(results[0].agent_id, "search-ops");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn query_limit_keeps_the_newest_matching_tail_in_append_order() {
    let dir = unique_chain_dir();
    let mut chain = MentisDb::open_with_key(&dir, "search-limit-eval").unwrap();

    for i in 0..5 {
        chain
            .append_thought(
                "planner",
                ThoughtInput::new(
                    ThoughtType::Summary,
                    format!("search baseline hit {i} for deterministic tail selection"),
                )
                .with_tags(["search"]),
            )
            .unwrap();
    }

    let results = chain.query(&ThoughtQuery::new().with_text("baseline hit").with_limit(2));
    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0].content,
        "search baseline hit 3 for deterministic tail selection"
    );
    assert_eq!(
        results[1].content,
        "search baseline hit 4 for deterministic tail selection"
    );
    assert!(results[0].index < results[1].index);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn indexed_filters_narrow_candidates_but_results_stay_in_append_order() {
    let dir = unique_chain_dir();
    let mut chain = MentisDb::open_with_key(&dir, "search-order-eval").unwrap();

    chain
        .append_thought(
            "planner",
            ThoughtInput::new(ThoughtType::Decision, "latency work item alpha")
                .with_tags(["search", "alpha"])
                .with_concepts(["retrieval"]),
        )
        .unwrap();
    chain
        .append_thought(
            "planner",
            ThoughtInput::new(ThoughtType::Insight, "latency work item beta")
                .with_tags(["search", "beta"])
                .with_concepts(["retrieval"]),
        )
        .unwrap();
    chain
        .append_thought(
            "planner",
            ThoughtInput::new(ThoughtType::Decision, "latency work item gamma")
                .with_tags(["search", "alpha"])
                .with_concepts(["retrieval"]),
        )
        .unwrap();

    let results = chain.query(
        &ThoughtQuery::new()
            .with_types(vec![ThoughtType::Decision])
            .with_tags_any(["alpha"])
            .with_concepts_any(["retr"])
            .with_text("latency"),
    );

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].content, "latency work item alpha");
    assert_eq!(results[1].content, "latency work item gamma");
    assert!(results[0].index < results[1].index);

    let _ = std::fs::remove_dir_all(&dir);
}
