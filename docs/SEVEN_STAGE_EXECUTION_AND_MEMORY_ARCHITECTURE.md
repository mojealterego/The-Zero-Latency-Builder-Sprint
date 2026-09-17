# Seven-Stage Execution Plan — Zero-Latency Builder Sprint

**Status:** Engineering plan and architecture decision record. This document does not claim that unimplemented components, benchmarks, or Moss integration are complete.

## Executive decision

Adopt a **bitemporal graph memory** as the canonical long-term memory model, with deterministic local retrieval as the latency-critical path. Treat Graph-of-Thought as a bounded planning representation, CoALA as a memory taxonomy/control model, HDC as an optional compact associative representation, and “holographic memory”, “G-memory”, and “digital genotype” as research/experimental concepts until each has a precise operational definition and benchmark. Do not combine these labels into one opaque subsystem.

The proposed architecture is hybrid:

1. **Canonical facts and events:** SQLite tables with explicit provenance, validity time, transaction time, confidence, and source.
2. **Graph projection:** typed entities and relations, rebuilt or updated transactionally from canonical records.
3. **Fast path:** exact IDs, metadata filters, lexical index, bounded graph traversal, capped top-k and token budget.
4. **Optional associative path:** HDC encodings for approximate candidate generation only; never authoritative and always traceable to source records.
5. **Planner:** bounded Graph-of-Thought DAG; nodes contain concise hypotheses/actions/evidence references, not hidden chain-of-thought. Persist only user-visible rationale summaries and operational traces.
6. **Policy/control:** CoALA-inspired separation of working, episodic, semantic, and procedural memory, with explicit read/write permissions and retention.
7. **Runtime adapter:** Moss remains behind a narrow port and is not represented as integrated until its official SDK/API contract and a real integration test are verified.

## Point 2 — Memory architecture

### 2.1 Bitemporal graph memory (canonical)

Every fact/relation records two independent timelines:

- **Valid time:** when the fact is true in the modeled world (`valid_from`, `valid_to`).
- **Transaction time:** when this system learned or stored the fact (`recorded_at`, `superseded_at`).

Recommended record fields: `id`, `subject_id`, `predicate`, `object_id` or `value_json`, `valid_from`, `valid_to`, `recorded_at`, `superseded_at`, `source_id`, `confidence`, `access_scope`, `content_hash`.

Required semantics:
- Corrections append a new version; they do not silently rewrite history.
- “What did we believe at time T?” filters by transaction time.
- “What was true at time T?” filters by valid time.
- Conflicting facts remain separately attributable; policy decides whether to surface conflict or apply a deterministic resolution rule.
- Deletion/retention requests must propagate to derived indexes and caches, subject to applicable audit requirements.

### 2.2 CoALA-inspired memory control

Use the categories as an operational taxonomy, not as a claim of exact reproduction of a paper implementation:

| Memory class | Purpose | Persistence | Access policy |
|---|---|---|---|
| Working | Current turn/task scratch state | Ephemeral | Current run only; bounded size |
| Episodic | Events, outcomes, interaction summaries | Durable, append-oriented | Provenance required; retention controls |
| Semantic | Normalized entities, facts, relations | Durable, bitemporal | Confidence and temporal filters |
| Procedural | Approved workflows, tool contracts, policies | Versioned durable | Explicit approval; never inferred from untrusted retrieved text |

All writes carry actor, reason/category, source, and idempotency key. Retrieval returns provenance and temporal scope with each item.

### 2.3 Graph-of-Thought (bounded planner representation)

Represent planning as a DAG with typed nodes: `question`, `candidate`, `evidence`, `action`, `verification`, `terminal`. Edges express `supports`, `contradicts`, `depends_on`, or `derived_from`.

Controls:
- Hard caps on nodes, depth, expansion count, wall-clock budget, and context tokens.
- Every factual claim node points to memory/source IDs.
- Contradictory evidence is preserved rather than collapsed into a single unsupported answer.
- Persist the compact decision trace, evidence references, and outcome—not private chain-of-thought.
- Deterministic pruning: remove dominated/unsupported branches; tie-break by stable IDs.

### 2.4 HDC (optional experimental candidate index)

Hyperdimensional computing may encode entity/attribute bundles into fixed-width vectors for fast approximate association. It is **not** a substitute for canonical facts or temporal filtering.

Gate before adoption:
- Define encoding, dimension, seed/version, binding/bundling operations, and collision behavior.
- Measure Recall@k, false-positive rate, memory footprint, encode/update cost, and p50/p95/p99 query latency.
- Test multilingual names, negation, temporal changes, and near-collisions.
- Return canonical record IDs and verify them against the graph/store before use.

### 2.5 “Holographic memory”, “G-memory”, “digital genotype”

These terms are underspecified in the current proposal and have multiple meanings across research and product contexts. They must not be treated as interchangeable established components.

- **Holographic memory:** define whether this means HDC-style distributed representations, a holographic reduced representation, or a different mechanism. Keep experimental behind a trait.
- **G-memory:** specify the source paper/system, data model, update semantics, and intended measurable advantage before implementation.
- **Digital genotype:** define precisely what is encoded (stable user-approved preferences, agent configuration, workflow traits, or something else), consent, mutability, portability, deletion, and leakage risks. Never infer sensitive traits or use it to profile users without explicit purpose and consent.

Until definitions and evidence exist, these remain named research tracks—not production dependencies.

### 2.6 RAG 2.0

Use the label only as a project shorthand for a measurable pipeline, not as a standardized protocol:

`ingest → normalize/provenance → temporal graph + lexical index → candidate retrieval → evidence verification → bounded planning → answer/action → trace/evaluation`

The MVP avoids mandatory embeddings/vector DBs. Add them only if a frozen labeled evaluation set demonstrates a material quality/latency trade-off improvement.

## Seven execution stages

### Stage 1 — Repository and evidence audit
- Freeze the base commit and inventory source, tests, CI, docs, licenses, and external dependencies.
- Record what exists versus what is proposed; map every claim to code, test, or benchmark evidence.
- Exit gate: auditable inventory and clean baseline CI.

### Stage 2 — Memory model and persistence
- Specify schemas for bitemporal facts/edges, provenance, source documents, and append-only events.
- Add migrations, transaction boundaries, uniqueness/idempotency constraints, and rebuildable indexes.
- Exit gate: temporal query tests, correction/history tests, restart persistence tests, and migration tests.

### Stage 3 — Deterministic retrieval vertical slice
- Implement exact-ID lookup, lexical retrieval, metadata/time filters, bounded graph expansion, and stable ranking.
- Enforce result and token budgets; return source IDs and temporal validity.
- Exit gate: deterministic fixtures and labeled Recall@k/MRR baseline.

### Stage 4 — Planner and policy control
- Implement bounded GoT DAG state, CoALA-inspired memory permissions, allowlisted tools, and untrusted-content isolation.
- Ensure cancellation, idempotency, and explicit state transitions.
- Exit gate: adversarial prompt-injection tests, budget-limit tests, and replayable traces.

### Stage 5 — Moss adapter and real workflow
- Verify official Moss package, version, auth, lifecycle, streaming, cancellation, and deployment constraints from authoritative documentation.
- Implement the narrow adapter only against the verified contract; keep a deterministic test double.
- Exit gate: a live integration test or clearly documented blocker; no mock presented as real integration.

### Stage 6 — Reliability, collaboration, and security
- Add event sequencing, conflict policy, duplicate handling, recovery/replay, payload validation, secret hygiene, and retention/deletion propagation.
- Exit gate: failure-injection tests, deterministic replay, and documented threat model.

### Stage 7 — Benchmarks, demo, and submission package
- Freeze corpus and labeled queries; record hardware, OS, compiler/runtime, corpus shape, warm/cold, concurrency, sample count, and exact commands.
- Report p50/p95/p99 separately for local retrieval, model/runtime, tools, and end-to-end workflow; include Recall@k/MRR and memory/disk costs.
- Produce a reproducible demo and submission narrative tied to verified artifacts only.
- Exit gate: benchmark reproducibility, CI pass, documented limitations, and no unsupported “zero latency” or sub-10-ms end-to-end claims.

## Required acceptance tests

1. A corrected fact preserves both the old transaction-time view and the new current view.
2. A query for a past valid-time interval returns only facts valid then, with provenance.
3. Duplicate event submission is idempotent; conflicting sequence numbers are rejected or reconciled by a documented policy.
4. Retrieval output is stable for identical input/index versions and bounded by configured top-k/token limits.
5. Retrieved instructions are treated as data; only approved procedural memory can authorize tools.
6. Planner terminates under node/depth/time limits and records a compact trace.
7. HDC, if enabled, can only nominate candidates; canonical verification is mandatory.
8. Data deletion/retention updates canonical storage and all derived indexes/caches.
9. Moss claims are backed by a real adapter test against a verified API contract.
10. Benchmark reports distinguish local retrieval latency from inference/network/tool latency.

## Risks and mitigations

- **Concept stacking:** too many named paradigms obscure what is implemented. Mitigation: one canonical store, explicit interfaces, gated experiments.
- **Temporal ambiguity:** valid-time and transaction-time are conflated. Mitigation: separate fields and test both query modes.
- **Approximate-memory hallucination:** HDC candidate similarity is mistaken for truth. Mitigation: source-backed canonical verification.
- **Planner cost explosion:** GoT branch expansion increases latency. Mitigation: strict budgets and deterministic pruning.
- **Privacy/profiling risk:** “digital genotype” can imply sensitive persistent profiling. Mitigation: explicit scope, consent, access controls, deletion, and no sensitive-trait inference.
- **Unsupported performance claims:** synthetic microbenchmarks are generalized. Mitigation: disclose conditions and report percentile distributions plus quality metrics.

## Current status

This file is a proposed execution and architecture plan. It does not certify that any of these seven stages has been completed. Implementation status must be updated only after code changes, tests, and CI evidence are inspected.