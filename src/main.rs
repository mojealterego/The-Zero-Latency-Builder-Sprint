use zero_latency_builder::{select_context, ContextRecord};

fn main() {
    let corpus = vec![
        ContextRecord { id: "architecture".into(), text: "Rust runtime with bounded local context retrieval".into() },
        ContextRecord { id: "transport".into(), text: "Zenoh can provide pub sub and query reply transport".into() },
        ContextRecord { id: "integration".into(), text: "Moss adapter must be verified against its official API".into() },
    ];

    let query = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let query = if query.is_empty() { "local context".to_string() } else { query };
    let selected = select_context(&query, &corpus, 3);

    println!("query: {query}");
    for item in selected {
        println!("{}\t{}\t{}", item.score, item.id, item.text);
    }
}
