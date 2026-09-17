use std::time::Instant;
use zero_latency_builder::{select_context, ContextRecord};

fn percentile(sorted: &[u128], percentile: usize) -> u128 {
    let index = ((sorted.len() - 1) * percentile).div_ceil(100);
    sorted[index]
}

fn main() {
    const RECORD_COUNT: usize = 1_000;
    const ITERATIONS: usize = 1_000;

    let records: Vec<ContextRecord> = (0..RECORD_COUNT)
        .map(|index| ContextRecord {
            id: format!("record-{index:04}"),
            text: format!(
                "document {index} local retrieval context topic-{} rust runtime",
                index % 25
            ),
        })
        .collect();
    let query = "local retrieval topic-7";
    let mut samples = Vec::with_capacity(ITERATIONS);

    for _ in 0..ITERATIONS {
        let start = Instant::now();
        let result = select_context(query, &records, 10);
        let elapsed = start.elapsed().as_nanos();
        std::hint::black_box(result);
        samples.push(elapsed);
    }

    samples.sort_unstable();
    let total: u128 = samples.iter().sum();
    let mean = total / samples.len() as u128;

    println!("benchmark: lexical retrieval");
    println!("records: {RECORD_COUNT}");
    println!("iterations: {ITERATIONS}");
    println!("query: {query}");
    println!("top_k: 10");
    println!("mean_ns: {mean}");
    println!("p50_ns: {}", percentile(&samples, 50));
    println!("p95_ns: {}", percentile(&samples, 95));
    println!("p99_ns: {}", percentile(&samples, 99));
    println!("note: single-process synthetic microbenchmark; not a production latency claim");
}
