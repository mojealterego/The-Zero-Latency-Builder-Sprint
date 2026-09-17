use std::time::Instant;
use zero_latency_builder::{select_context, ContextRecord};

fn percentile(sorted: &[u128], percentile: usize) -> u128 {
    let index = ((sorted.len() - 1) * percentile).div_ceil(100);
    sorted[index]
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn main() {
    let record_count = env_usize("BENCH_RECORDS", 1_000);
    let warmup_iterations = env_usize("BENCH_WARMUP", 200);
    let iterations = env_usize("BENCH_ITERATIONS", 1_000);
    let top_k = env_usize("BENCH_TOP_K", 10);

    let records: Vec<ContextRecord> = (0..record_count)
        .map(|index| ContextRecord {
            id: format!("record-{index:04}"),
            text: format!(
                "document {index} local retrieval context topic-{} rust runtime",
                index % 25
            ),
        })
        .collect();
    let query =
        std::env::var("BENCH_QUERY").unwrap_or_else(|_| "local retrieval topic-7".to_string());

    for _ in 0..warmup_iterations {
        std::hint::black_box(select_context(&query, &records, top_k));
    }

    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        let result = select_context(&query, &records, top_k);
        let elapsed = start.elapsed().as_nanos();
        std::hint::black_box(result);
        samples.push(elapsed);
    }

    samples.sort_unstable();
    let total: u128 = samples.iter().sum();
    let mean = total / samples.len() as u128;

    println!("benchmark: lexical retrieval");
    println!("records: {record_count}");
    println!("warmup_iterations: {warmup_iterations}");
    println!("measured_iterations: {iterations}");
    println!("query: {query}");
    println!("top_k: {top_k}");
    println!("mean_ns: {mean}");
    println!("p50_ns: {}", percentile(&samples, 50));
    println!("p95_ns: {}", percentile(&samples, 95));
    println!("p99_ns: {}", percentile(&samples, 99));
    println!("rust_version: record with `rustc --version`");
    println!("host: record OS/CPU/RAM alongside results");
    println!("note: single-process synthetic microbenchmark; not a production latency claim");
}
