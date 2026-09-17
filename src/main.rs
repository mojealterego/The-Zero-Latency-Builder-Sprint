use zero_latency_builder::{
    policy::{authorize, Decision, ExecutionMode},
    replay::{AuditEvent, ReplayLedger},
    select_context, ContextRecord,
};

fn main() {
    let corpus = vec![
        ContextRecord {
            id: "architecture".into(),
            text: "Rust runtime with bounded local context retrieval".into(),
        },
        ContextRecord {
            id: "transport".into(),
            text: "Zenoh can provide pub sub and query reply transport".into(),
        },
        ContextRecord {
            id: "integration".into(),
            text: "Moss adapter must be verified against its official API".into(),
        },
    ];

    let query = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let query = if query.is_empty() {
        "local context".to_string()
    } else {
        query
    };
    let selected = select_context(&query, &corpus, 3);

    println!("query: {query}");
    for item in selected {
        println!("context\t{}\t{}\t{}", item.score, item.id, item.text);
    }

    // Demonstrate policy evaluation only: no external tool is called.
    let decision = authorize(
        "notes.search",
        &["notes.search"],
        false,
        false,
        ExecutionMode::DryRun,
    );
    println!("policy_decision: {decision:?}");

    let mut ledger = ReplayLedger::default();
    if decision == Decision::AllowDryRun {
        let event = AuditEvent {
            event_id: "demo-0001".into(),
            idempotency_key: "demo-search-0001".into(),
            action: "notes.search".into(),
            policy_version: "demo-v1".into(),
            outcome: "dry_run_only".into(),
        };
        match ledger.record(event) {
            Ok(()) => println!("audit_events: {}", ledger.len()),
            Err(error) => eprintln!("audit_record_error: {error:?}"),
        }
    }
    println!("external_side_effects: none");
}
