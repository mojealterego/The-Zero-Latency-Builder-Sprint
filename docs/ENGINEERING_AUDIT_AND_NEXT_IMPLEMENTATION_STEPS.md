# Engineering Audit and Next Implementation Steps

**Status:** repository inspection and design record; not a completed implementation or test report.

## Scope inspected

The current Rust library exposes three modules (`policy`, `replay`, `runtime`) and a deterministic lexical `select_context` baseline. The inspected source is deliberately small and currently has no demonstrated persistence or external tool execution.

## Findings

### Retrieval (`src/lib.rs`)
- Uses distinct, lowercased alphanumeric tokens and exact whole-token overlap.
- Ranks by overlap count, then record ID; truncates to `limit`.
- This is a useful deterministic baseline, not semantic retrieval, vectorless graph retrieval, or a Moss integration.
- Missing product-level controls include input-size limits, score explanation, freshness/provenance weighting, and a defined duplicate-ID policy.

### Policy (`src/policy.rs`)
- Exact allowlist matching and fail-closed rejection of unknown actions are present.
- Dry-run is a decision outcome only; it does not execute a tool.
- Approval is represented as a boolean supplied by the caller. Production use must bind approval to a specific action, payload hash, principal, expiry, and policy version rather than trusting an unscoped boolean.

### Replay (`src/replay.rs`)
- The ledger is in-memory and preserves append order.
- It rejects blank IDs, duplicate event IDs, and reuse of an idempotency key for a different event.
- It is not durable across process restart, and replay returns records without repeating side effects.

### Runtime (`src/runtime.rs`)
- `decide_and_record` authorizes and records an outcome only.
- It does not invoke external tools. A future executor must remain a separate, explicitly gated component.

### Bitemporal SQL migration
- `migrations/0001_bitemporal_memory.sql` defines the initial SQLite memory/source/event schema.
- The migration has not been executed in this audit. SQL dialect compatibility, trigger behavior, transaction semantics, and migration rollback/recovery remain unverified.

## Recommended implementation sequence

### Step A — establish reproducible CI first
1. Confirm the repository's actual Rust toolchain and current workflow files.
2. Run `cargo fmt --check`, `cargo test --all-targets`, and `cargo clippy --all-targets -- -D warnings` in CI.
3. Add a SQLite migration smoke test against a temporary database, including schema creation and trigger assertions.
4. Record toolchain, OS, SQLite version, and command results in CI artifacts.

**Acceptance:** a clean CI run from a fresh checkout; no claim of passing until the workflow reports success.

### Step B — define a durable memory boundary
1. Introduce a small `MemoryStore` interface for append, correction/supersession, and as-of retrieval.
2. Keep SQLite as the initial canonical store; avoid adding vector/embedding dependencies before baseline measurements.
3. Use parameterized SQL and explicit transactions. A correction must close the prior transaction-time version and append a new fact/event atomically.
4. Enforce source/provenance references and validate time intervals (`valid_from < valid_to` when bounded; transaction intervals likewise).
5. Define idempotency semantics for both fact writes and event writes.

**Acceptance:** tests cover current-time and historical valid-time queries, correction without destructive overwrite, duplicate idempotency, rollback on partial failure, and isolation between scopes.

### Step C — harden retrieval while preserving baseline
1. Preserve the current lexical selector as a named baseline for comparisons.
2. Add a result explanation (matched terms and deterministic tie-break) and explicit duplicate-ID handling.
3. Add configurable bounded query/record lengths and maximum candidate count.
4. Add temporal and access-scope filtering before ranking; never rank unauthorized records and filter afterward.
5. Benchmark recall@k, latency p50/p95, memory use, and corpus-size scaling on a checked-in reproducible fixture.

**Acceptance:** deterministic tests, adversarial scope tests, and benchmark output tied to a commit and machine profile.

### Step D — make policy approvals non-transferable
1. Replace bare approval booleans at the live-execution boundary with a typed approval record.
2. Bind approval to principal, action, canonical payload digest, policy version, expiry, and single-use nonce.
3. Keep unknown actions denied; treat missing/expired/mismatched approvals as deny.
4. Audit both allow and deny decisions with redaction rules for secrets and personal data.

**Acceptance:** tests reject altered payloads, expired approvals, replayed nonces, mismatched principals, and policy-version drift.

### Step E — isolate any future tool execution
1. Keep decision/audit code independent from the executor.
2. Require explicit live mode, allowlisted action, scoped approval where applicable, timeout, cancellation, and bounded output.
3. Make idempotency behavior explicit per tool; never assume a network side effect can be safely replayed.
4. Use fake tools in tests; no real external side effects in unit or CI tests.

**Acceptance:** integration tests prove denied calls never reach the executor and that retry behavior is bounded and auditable.

### Step F — verify Moss integration as an evidence-based adapter
1. Record the exact Moss API/protocol, version, authentication method, latency target, and failure semantics from authoritative project materials before implementation.
2. Implement behind an adapter interface, with timeout and fallback to local lexical retrieval.
3. Do not describe the adapter as integrated until a reproducible end-to-end test succeeds against the intended environment.

**Acceptance:** protocol contract, mocked contract tests, and a separately documented live smoke test.

## Immediate risk register

| Risk | Current consequence | Mitigation |
|---|---|---|
| In-memory-only ledger | Audit history disappears on restart | Durable event store and restart tests |
| Boolean approval | Approval may be reused for a different request | Payload-bound, expiring, single-use approval |
| Lexical-only retrieval | Weak recall for paraphrases and multilingual queries | Measure baseline, then evaluate bounded alternatives |
| Unexecuted migration | Schema and trigger assumptions unverified | SQLite migration smoke tests |
| Unverified Moss contract | Integration may target incorrect assumptions | Verify protocol/version before coding |
| No confirmed CI evidence in this audit | Build health is unknown | Inspect workflow and require green CI |

## Non-claims

This document does not claim that SQLite is wired into Rust, that migrations have run, that CI is green, that Moss is connected, or that external tools can execute. Those require implementation and observed test evidence.
