# Stage 3 — Persistent Memory and Policy Hardening

**Status:** implementation specification; not a claim that SQLite persistence is implemented.

## Objective

Move the prototype from process-local demonstrations toward a durable, auditable memory boundary while keeping external execution disabled by default. Stage 3 is complete only when the acceptance gates below are implemented and exercised in CI.

## Work package A — Durable event ledger

1. Introduce a storage abstraction around the existing replay ledger; keep the in-memory implementation for unit tests.
2. Add a SQLite-backed implementation using explicit transactions.
3. Enforce uniqueness for `event_id` and `idempotency_key` in the database, not only in Rust memory.
4. Make append atomic: either the event and its idempotency record commit together, or neither does.
5. On startup, recover by reading the durable ledger in deterministic sequence order. Never silently discard malformed or duplicate records.
6. Define schema versioning and migration behavior; migration failure must abort startup with a useful error.

### Acceptance tests

- Reopening the database preserves event order and content.
- Repeated event ID is rejected after process restart.
- Reused idempotency key with a different event is rejected after restart.
- Simulated transaction failure leaves no partial event.
- Concurrent writers cannot create duplicate IDs or keys.
- Database lock and I/O errors propagate; no success-shaped fallback.

## Work package B — Bitemporal memory correctness

The initial schema distinguishes valid time (`valid_from`, `valid_to`) from transaction time (`recorded_at`, `superseded_at`). Preserve that distinction throughout queries and writes.

1. Require UTC-normalized, documented timestamp representation.
2. Validate interval shape: when both bounds exist, `valid_from < valid_to` and `recorded_at < superseded_at`.
3. Define whether overlapping valid-time facts are allowed per subject/predicate; enforce the chosen rule transactionally.
4. Corrections append a new version and close the prior transaction-time interval; do not rewrite historical values in place.
5. Queries must state whether they ask “what was valid then?” or “what did the system know then?”
6. Treat confidence as a calibrated/opaque value until a calibration method exists; never interpret it as truth probability by default.

### Acceptance tests

- Historical valid-time and transaction-time queries return distinct expected answers after a correction.
- Invalid intervals are rejected.
- A correction preserves the prior version and its provenance.
- Same idempotency key cannot create two facts.
- Scope filters prevent cross-scope reads.

## Work package C — Authorization hardening

The current policy prototype uses an allowlist and a boolean approval. Before any live side effect, replace the boolean with a structured, verifiable approval record.

Approval record minimum fields:

- principal / approving identity
- exact action and canonical payload digest
- policy version
- issued-at and expiry
- unique approval ID / nonce
- environment or resource scope
- decision and optional rationale

Rules:

1. Deny by default, including unknown actions and malformed inputs.
2. Bind approval to the exact canonical payload; any payload change invalidates it.
3. Reject expired, replayed, wrong-principal, wrong-scope, and wrong-policy approvals.
4. Keep dry-run and live execution paths distinct at the type/API boundary.
5. Do not add a live executor in this stage. The runtime remains decision-and-record only.
6. Redact secrets and sensitive payload fields from logs; store digests and safe summaries where possible.

### Acceptance tests

- Unknown action, absent approval, expired approval, wrong digest, wrong scope, and replay all deny.
- A valid approval only authorizes its exact action/payload within its lifetime.
- Dry-run cannot invoke a side-effect adapter.
- Audit records contain policy version and decision, without secrets.

## Work package D — Retrieval determinism and measurement

The lexical selector is a baseline, not semantic search. Preserve it as a deterministic fallback and establish reproducible measurement before introducing embeddings or HDC.

Record for each benchmark run:

- corpus size and stable corpus hash
- query-set hash
- candidate and selected counts
- latency distribution (at least p50, p95, p99)
- recall@k against a reviewed relevance set
- runtime/compiler and machine metadata

Do not claim sub-10ms performance without specifying hardware, corpus, query distribution, warm/cold state, and percentile. Do not compare latency across runs with materially different conditions.

## Work package E — CI and repository hygiene

1. Retain formatting, Clippy, tests, and benchmark smoke checks.
2. Ensure only one canonical workflow owns each required check, or document why multiple workflows are intentionally distinct.
3. Add migration tests using temporary databases and deterministic fixtures.
4. Add dependency auditing only after dependencies are introduced; pin or constrain versions deliberately.
5. Keep benchmark smoke execution separate from performance claims.

## Suggested implementation sequence

1. Inspect and consolidate current workflow definitions; verify latest Actions status.
2. Introduce storage traits and error types without changing current behavior.
3. Add SQLite dependency and persistence adapter in a dedicated commit.
4. Add migration runner and migration tests.
5. Add structured approval type and negative tests; keep live execution unavailable.
6. Add deterministic benchmark metadata and a small reviewed relevance fixture.
7. Update architecture and README to distinguish implemented, tested, and planned capabilities.

## Exit criteria

Stage 3 is not complete until all of the following are true:

- Persistent replay and idempotency survive process restart.
- SQLite migration and temporal invariants are tested in CI.
- Authorization is payload-bound and replay-resistant.
- No external side effects are possible through the current runtime.
- CI is green on the resulting commit, with the exact run linked in the project record.
- Documentation labels prototype, implemented, tested, and experimental capabilities accurately.

## Explicit non-claims

This document does not establish that the database migration has been executed, that SQLite persistence exists, that approvals are cryptographically verified, that retrieval meets a latency target, or that any external Moss/Zenoh integration has been validated.