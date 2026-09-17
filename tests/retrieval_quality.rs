use zero_latency_builder::{select_context, ContextRecord};

fn fixture() -> Vec<ContextRecord> {
    vec![
        ContextRecord { id: "noise".into(), text: "unrelated gardening notes".into() },
        ContextRecord { id: "design".into(), text: "local context retrieval design".into() },
        ContextRecord { id: "runtime".into(), text: "Rust runtime and local retrieval".into() },
        ContextRecord { id: "policy".into(), text: "tool execution policy and approvals".into() },
    ]
}

#[test]
fn relevant_documents_are_retrieved_in_expected_top_two() {
    let ranked = select_context("local retrieval", &fixture(), 2);
    let ids: Vec<&str> = ranked.iter().map(|item| item.id.as_str()).collect();
    assert_eq!(ids, vec!["design", "runtime"]);
}

#[test]
fn irrelevant_documents_do_not_enter_results() {
    let ranked = select_context("approval policy", &fixture(), 10);
    assert!(ranked.iter().all(|item| item.id != "noise"));
    assert!(ranked.iter().any(|item| item.id == "policy"));
}

#[test]
fn top_k_is_a_hard_output_bound() {
    let ranked = select_context("local retrieval runtime context", &fixture(), 1);
    assert_eq!(ranked.len(), 1);
}
