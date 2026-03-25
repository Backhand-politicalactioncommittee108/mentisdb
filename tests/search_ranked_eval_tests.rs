use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use mentisdb::{
    MentisDb, RankedSearchBackend, RankedSearchQuery, ThoughtInput, ThoughtQuery, ThoughtType,
};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_chain_dir() -> PathBuf {
    let n = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "mentisdb_search_ranked_eval_test_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn ranked_search_filter_remains_authoritative_before_lexical_ranking() {
    let dir = unique_chain_dir();
    let mut chain = MentisDb::open_with_key(&dir, "ranked-filter-authority-eval").unwrap();

    chain
        .append_thought(
            "planner-a",
            ThoughtInput::new(
                ThoughtType::Decision,
                "Navigator lexical signal should survive filtering.",
            )
            .with_tags(["search", "keep"]),
        )
        .unwrap();

    chain
        .append_thought(
            "planner-b",
            ThoughtInput::new(
                ThoughtType::Decision,
                "Navigator lexical signal should be filtered out first.",
            )
            .with_tags(["archive"]),
        )
        .unwrap();

    let ranked = chain.query_ranked(
        &RankedSearchQuery::new()
            .with_filter(ThoughtQuery::new().with_tags_any(["search"]))
            .with_text("navigator lexical"),
    );

    assert_eq!(ranked.backend, RankedSearchBackend::Lexical);
    assert_eq!(ranked.total_candidates, 1);
    assert_eq!(ranked.hits.len(), 1);
    assert_eq!(ranked.hits[0].thought.agent_id, "planner-a");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ranked_search_blank_text_falls_back_to_heuristic_without_dropping_candidates() {
    let dir = unique_chain_dir();
    let mut chain = MentisDb::open_with_key(&dir, "ranked-blank-text-eval").unwrap();

    chain
        .append_thought(
            "agent",
            ThoughtInput::new(
                ThoughtType::Insight,
                "Older high-signal ranked search note.",
            )
            .with_importance(1.0)
            .with_confidence(1.0)
            .with_tags(["search"]),
        )
        .unwrap();
    chain
        .append_thought(
            "agent",
            ThoughtInput::new(
                ThoughtType::Insight,
                "Newer lower-signal ranked search note.",
            )
            .with_importance(0.1)
            .with_confidence(0.1)
            .with_tags(["search"]),
        )
        .unwrap();

    let ranked = chain.query_ranked(
        &RankedSearchQuery::new()
            .with_filter(ThoughtQuery::new().with_tags_any(["search"]))
            .with_text("   ")
            .with_limit(10),
    );

    assert_eq!(ranked.backend, RankedSearchBackend::Heuristic);
    assert_eq!(ranked.total_candidates, 2);
    assert_eq!(ranked.hits.len(), 2);
    assert_eq!(
        ranked.hits[0].thought.content,
        "Older high-signal ranked search note."
    );
    assert_eq!(
        ranked.hits[1].thought.content,
        "Newer lower-signal ranked search note."
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ranked_search_reports_pre_limit_candidate_count_and_truncates_after_ranking() {
    let dir = unique_chain_dir();
    let mut chain = MentisDb::open_with_key(&dir, "ranked-limit-eval").unwrap();

    for (i, importance) in [0.0_f32, 1.0, 0.5, 0.1].into_iter().enumerate() {
        chain
            .append_thought(
                "agent",
                ThoughtInput::new(
                    ThoughtType::Decision,
                    format!("ranked lexical candidate {i} with shared tokens"),
                )
                .with_tags(["search", "ranked"])
                .with_importance(importance),
            )
            .unwrap();
    }

    let ranked = chain.query_ranked(
        &RankedSearchQuery::new()
            .with_filter(ThoughtQuery::new().with_tags_any(["ranked"]))
            .with_text("ranked lexical candidate")
            .with_limit(2),
    );

    assert_eq!(ranked.backend, RankedSearchBackend::Lexical);
    assert_eq!(ranked.total_candidates, 4);
    assert_eq!(ranked.hits.len(), 2);
    assert!(ranked.hits[0].score.total >= ranked.hits[1].score.total);
    assert_eq!(
        ranked.hits[0].thought.content,
        "ranked lexical candidate 1 with shared tokens"
    );
    assert_eq!(
        ranked.hits[1].thought.content,
        "ranked lexical candidate 2 with shared tokens"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
