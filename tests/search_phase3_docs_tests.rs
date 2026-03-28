#[test]
fn readme_marks_phase3_as_complete_and_optional() {
    let readme = include_str!("../README.md");

    assert!(readme.contains("MentisDB now exposes an additive Phase 3 vector sidecar surface"));
    assert!(readme.contains("embeddings remain optional"));
    assert!(readme.contains("vector state lives in a rebuildable sidecar"));
    assert!(
        readme.contains("vector hits surface whether they came from a `Fresh` or stale sidecar")
    );
    assert!(!readme.contains("has **not started yet**"));
}

#[test]
fn phase3_plan_captures_semantic_acceptance_criteria() {
    let plan = include_str!("../WORLDCLASS_SEARCH_PLAN.md");

    assert!(plan.contains("Phase 3 is complete in the core crate."));
    assert!(plan.contains("Status: complete in the core crate on `master`"));
    assert!(plan.contains("### Acceptance Criteria"));
    assert!(plan.contains(
        "A chain with Phase 3 disabled behaves normally with lexical and graph retrieval only"
    ));
    assert!(plan.contains("model_id"));
    assert!(plan.contains("embedding version"));
    assert!(plan.contains(
        "Search results can tell callers whether a vector hit came from fresh or stale embeddings."
    ));
    assert!(plan.contains("Sidecar corruption or deletion degrades semantic/vector retrieval only"));
}
