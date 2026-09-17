# The Zero Latency Builder Sprint

A prototype for exploring low-latency, local-first context retrieval and reliable agent workflows for the **YC Fall 2026 × Moss: The Zero Latency Builder Sprint**.

> **Current status:** early Rust prototype. This repository does not yet contain a verified Moss integration, Zenoh transport, persistent event log, or production latency results. Do not interpret the project name or sprint goals as measured performance claims.

## Current implementation

- Rust crate (`zero-latency-builder`, edition 2021).
- A deterministic lexical context selector in `src/lib.rs`.
- A small command-line demonstration in `src/main.rs` using an embedded synthetic corpus.
- Unit tests for retrieval behavior.
- A standalone synthetic microbenchmark in `benches/retrieval.rs` that reports mean and p50/p95/p99 timing for a 1,000-record corpus over 1,000 iterations.
- GitHub Actions workflow for formatting and tests.

## Requirements

- Rust stable toolchain, including Cargo and rustfmt.

## Run the CLI

```bash
cargo run -- "local context"
```

Pass another query as the argument to search the demo corpus.

## Run checks

```bash
cargo fmt --all -- --check
cargo test --all-targets
```

## Run the microbenchmark

```bash
cargo bench --bench retrieval
```

The benchmark uses synthetic in-memory data and measures only the local lexical selector in a single process. Results depend on the machine and build environment. Record the hardware, Rust version, build mode, corpus/query details, and run conditions when publishing results. These measurements do **not** represent end-to-end agent latency or Moss performance.

## Architecture status

| Component | Status |
|---|---|
| Rust lexical retrieval baseline | Implemented in prototype |
| CLI demo | Implemented with embedded sample records |
| Unit tests / formatting CI | Configured; see GitHub Actions for the latest run |
| Synthetic retrieval benchmark | Added; actual benchmark output must be captured separately |
| Moss retrieval adapter | Not implemented; official API/version must be verified first |
| Zenoh transport | Not implemented |
| Persistent storage / event log / replay | Not implemented |
| Policy-gated tool execution | Not implemented |
| End-to-end latency and retrieval-quality evaluation | Not measured |

## Development principles

1. Keep measured results separate from targets and hypotheses.
2. Report p50/p95/p99 with workload, hardware, software versions, and warm/cold conditions.
3. Evaluate retrieval quality as well as speed (for example, Recall@k and MRR on a documented test set).
4. Treat retries, side effects, idempotency, authorization, and audit/replay as explicit reliability requirements before connecting tools.
5. Integrate Moss only through a real adapter using its verified, current interface; transport alone is not a substitute for Moss retrieval.

## Project progress

See [`POSTĘP.md`](POST%C4%98P.md) for the detailed stage-by-stage progress record and outstanding work.

## Disclaimer

This is a work in progress and a prototype. No production-readiness, sub-10-ms, or 1-ms end-to-end performance claim is made.