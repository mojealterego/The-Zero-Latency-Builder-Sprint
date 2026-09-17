# Benchmark protocol

## What the current benchmark measures

`cargo bench --bench retrieval` measures the synchronous lexical selector in `src/lib.rs` over a generated in-memory corpus. It includes tokenization, per-record token-set construction, scoring, sorting, and result cloning. It does **not** measure Zenoh transport, serialization, Moss retrieval, model inference, concurrency, or end-to-end request latency.

## Reproduce

```bash
rustc --version
cargo bench --bench retrieval
BENCH_RECORDS=10000 BENCH_WARMUP=1000 BENCH_ITERATIONS=5000 BENCH_TOP_K=10 cargo bench --bench retrieval
```

Record the exact command, commit SHA, Rust version, OS/kernel, CPU model, RAM, power mode, and whether the machine was otherwise idle. Preserve the full stdout output with the results.

## Required report fields

- Corpus size and generation method
- Query and `top_k`
- Warm-up and measured iteration counts
- Mean, p50, p95, and p99 in nanoseconds
- Hardware/software environment and commit SHA
- Retrieval quality on a fixed labeled query set (at minimum Recall@k; add MRR when relevant)
- Whether measurements are warm-cache, cold-start, single-process, or concurrent

## Interpretation rules

- Do not describe this microbenchmark as production, end-to-end, semantic-search, or Moss latency.
- Nanosecond units are a reporting resolution, not proof that a full request completes in nanoseconds.
- Do not claim a fixed speedup without running the old and new implementations under the same controlled workload and reporting distributions.
- `ArcSwap`, binary serialization, token hashes, SIMD, and CPU-specific compiler flags are hypotheses to benchmark—not automatic guarantees of lower total latency.
- Hash equality can collide and does not provide semantic understanding. Any hash-based retrieval must define collision handling and preserve a correctness/quality test set.

## Next benchmark stages

1. Add a pre-tokenized/indexed selector and compare it against the current baseline.
2. Add corpus/query fixtures with expected relevant IDs and report Recall@k/MRR.
3. Add concurrent load tests separately from the single-thread microbenchmark.
4. Measure transport and serialization independently before claiming end-to-end latency.
5. Integrate Moss only after its supported API and version are verified; then benchmark the actual integration.
