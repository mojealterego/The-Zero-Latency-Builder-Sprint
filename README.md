# The Zero Latency Builder Sprint

A prototype exploring low-latency, local-first context retrieval and reliable agent workflows for the **YC Fall 2026 × Moss: The Zero Latency Builder Sprint**.

> **Status:** early Rust prototype. The repository does not yet contain a verified Moss integration, Zenoh transport, persistent event log, policy-gated tool execution, or production latency results. The project name and sprint goals are not measured performance claims.

## What exists today

- Rust crate (`zero-latency-builder`, edition 2021).
- Deterministic lexical context selection in `src/lib.rs`.
- CLI demonstration in `src/main.rs` using a small embedded synthetic corpus.
- Unit tests covering core retrieval behavior.
- Synthetic microbenchmark in `benches/retrieval.rs` reporting mean and p50/p95/p99 for a 1,000-record corpus over 1,000 iterations.
- GitHub Actions workflow checking Rust formatting and running tests.

The selector is a **lexical baseline**, not semantic search or BM25. It is not currently backed by a persistent index.

## Requirements

- Rust stable toolchain with Cargo and rustfmt.
- Git, if you are cloning the repository.

## Quick start

Clone the repository and enter its directory:

```bash
git clone https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint.git
cd The-Zero-Latency-Builder-Sprint
```

Run the CLI with its default query:

```bash
cargo run
```

Or provide a query:

```bash
cargo run -- "local context"
```

The CLI searches only its built-in demonstration records; it does not query an external service or a user's files.

## Development checks

Check formatting:

```bash
cargo fmt --all -- --check
```

Run tests across targets:

```bash
cargo test --all-targets
```

Run both before submitting changes. The GitHub Actions workflow runs formatting and tests on pushes and pull requests targeting `main`, and supports manual dispatch.

## Microbenchmark

Run the standalone benchmark:

```bash
cargo bench --bench retrieval
```

It measures only the in-process lexical selector against synthetic in-memory records. It is not an end-to-end agent benchmark and does not measure Moss, transport, storage, or tool execution.

When recording results, include:

- CPU, memory, operating system, and Rust version;
- exact commit and build mode;
- corpus size and construction method;
- query, top-k, iteration count, and warm/cold conditions;
- mean, p50, p95, and p99, plus any relevant outliers.

Do not compare numbers from different environments as if they were directly equivalent. No latency result is currently claimed by this README; capture the actual command output before publishing measurements.

## Architecture and implementation status

| Component | Status |
|---|---|
| Rust lexical retrieval baseline | Implemented as a prototype |
| CLI demo | Implemented with embedded sample records |
| Unit tests and formatting CI | Configured; inspect the latest GitHub Actions run for its result |
| Synthetic retrieval benchmark | Added; benchmark output still needs to be captured and documented |
| Moss retrieval adapter | Not implemented; verify the official API and version before integration |
| Zenoh transport | Not implemented |
| Persistent storage, event log, and replay | Not implemented |
| Policy-gated tool execution | Not implemented |
| End-to-end latency and retrieval-quality evaluation | Not measured |

## Engineering principles

1. Separate measured results from targets, hypotheses, and design intentions.
2. Report p50/p95/p99 with workload, hardware, software versions, and warm/cold conditions.
3. Evaluate retrieval quality as well as speed (for example, Recall@k and MRR on a documented test set).
4. Treat authorization, retries, side effects, idempotency, and audit/replay as explicit reliability requirements before connecting tools.
5. Integrate Moss through a real adapter using its verified current interface; transport alone is not a substitute for Moss retrieval.
6. Add infrastructure only when it supports a defined requirement and can be measured independently.

## Roadmap

1. Capture and document a reproducible baseline benchmark.
2. Verify sprint requirements and the official Moss API, then implement and test a real retrieval adapter.
3. Define a versioned event contract and deterministic replay tests.
4. Add policy-gated tool execution with explicit authorization and idempotency behavior.
5. Evaluate transport options, including Zenoh if justified, and measure their overhead separately.
6. Run retrieval-quality and end-to-end evaluations before making performance claims.

## Progress log

See [`POSTĘP.md`](POST%C4%98P.md) for the stage-by-stage record, decisions, and outstanding work.

## Disclaimer

This is a work in progress and a prototype. No production-readiness, sub-10-ms, or 1-ms end-to-end performance claim is made.