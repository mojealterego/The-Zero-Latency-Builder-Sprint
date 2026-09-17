# Deterministic Replay and Audit Contract

Status: design contract; not yet implemented or security-validated.

## Goals

- Reconstruct what the runtime decided from an append-only event sequence.
- Separate recorded facts from replayed decisions.
- Make tool side effects visible and prevent replay from executing them again.

## Event envelope

Every event should carry:

- `schema_version`: integer, initially `1`.
- `run_id`, `event_id`, `sequence`: stable run identifier, unique event identifier, monotonic per-run sequence.
- `timestamp_utc`: RFC 3339 UTC timestamp for audit only; ordering uses `sequence`.
- `kind`: one of `input_received`, `context_selected`, `policy_decision`, `tool_requested`, `tool_result`, `run_completed`, `run_failed`.
- `payload`: kind-specific validated object.
- `previous_event_hash` and `event_hash`: integrity-chain fields, computed over canonical serialized event data.

## Replay rules

1. Read events in strictly increasing sequence order; reject duplicates, gaps, unknown schema versions, and invalid hashes.
2. Recompute pure retrieval/policy decisions from captured inputs and versioned configuration; compare with recorded outputs.
3. Treat tool results as immutable recorded observations. Replay must never invoke a real external tool.
4. Mark any nondeterministic dependency (model output, clock, random seed, external response) as an explicit captured input.
5. Report the first divergence with event ID, expected value, actual value, and component version.

## Side-effect safety

- Every side-effecting tool request requires a unique idempotency key and a policy decision tied to the exact canonical request hash.
- A replay mode must enforce `side_effects=disabled` at the executor boundary, not merely in the UI.
- Approval records bind to actor, request hash, allowed operation, expiry, and one-time nonce.
- Never store credentials or secret-bearing raw headers in the event log; redact before persistence.

## Minimum acceptance tests

- Valid event stream replays to the same deterministic decisions.
- Changed event payload, reordered sequence, duplicate event ID, or broken hash is rejected.
- Replay records tool results without invoking an executor.
- Approval for request A cannot authorize modified request B.
- Missing/expired approval blocks side-effect execution.

## Current status

This document specifies intended behavior only. No event store, hash-chain verifier, executor gate, or replay test has been implemented by this document.