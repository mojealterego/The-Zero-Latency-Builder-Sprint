# Stage 2 — Bitemporal Memory Implementation Specification

**Status:** Specification only. No persistence implementation or tests are claimed by this document.

## Goal

Add durable, auditable memory without changing the existing deterministic retrieval contract until migrations and tests pass. SQLite is the proposed first adapter; domain semantics must remain independent of SQLite.

## Scope for the first vertical slice

1. Source/provenance records.
2. Bitemporal facts and typed relations.
3. Append-oriented agent events with idempotency.
4. Queries by valid time and transaction time.
5. Transactional correction semantics.
6. Rebuildable lexical/graph projections.

HDC, holographic representations, distributed synchronization, and inferred digital-genotype profiles are explicitly out of scope for this slice.

## Temporal semantics

Use UTC Unix microseconds (signed 64-bit integer) consistently in storage. Intervals are half-open: `[from, to)`. A null `*_to` means open-ended.

- `valid_from`, `valid_to`: when the modeled assertion applies in the world.
- `recorded_from`, `recorded_to`: when this database version is considered current knowledge.
- A correction closes the prior transaction-time interval and inserts a new version in one transaction. It must not erase the old version.
- A retraction is represented as a new assertion/status event; retention deletion is a separate operation with explicit audit policy.

## Proposed SQLite schema

```sql
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS sources (
  source_id TEXT PRIMARY KEY,
  source_type TEXT NOT NULL,
  locator TEXT,
  content_hash TEXT,
  captured_at INTEGER NOT NULL,
  trust_label TEXT NOT NULL DEFAULT 'unverified',
  metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS memory_facts (
  fact_id TEXT PRIMARY KEY,
  subject_id TEXT NOT NULL,
  predicate TEXT NOT NULL,
  object_id TEXT,
  value_json TEXT,
  valid_from INTEGER NOT NULL,
  valid_to INTEGER,
  recorded_from INTEGER NOT NULL,
  recorded_to INTEGER,
  source_id TEXT NOT NULL REFERENCES sources(source_id),
  confidence REAL NOT NULL CHECK(confidence >= 0.0 AND confidence <= 1.0),
  access_scope TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'asserted',
  content_hash TEXT NOT NULL,
  CHECK(valid_to IS NULL OR valid_to > valid_from),
  CHECK(recorded_to IS NULL OR recorded_to > recorded_from),
  CHECK((object_id IS NOT NULL) OR (value_json IS NOT NULL))
);

CREATE INDEX IF NOT EXISTS idx_facts_subject_predicate
  ON memory_facts(subject_id, predicate);
CREATE INDEX IF NOT EXISTS idx_facts_valid_time
  ON memory_facts(valid_from, valid_to);
CREATE INDEX IF NOT EXISTS idx_facts_recorded_time
  ON memory_facts(recorded_from, recorded_to);

CREATE TABLE IF NOT EXISTS agent_events (
  event_id TEXT PRIMARY KEY,
  idempotency_key TEXT NOT NULL UNIQUE,
  stream_id TEXT NOT NULL,
  sequence_no INTEGER NOT NULL,
  actor_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  recorded_at INTEGER NOT NULL,
  UNIQUE(stream_id, sequence_no)
);
CREATE INDEX IF NOT EXISTS idx_events_stream_sequence
  ON agent_events(stream_id, sequence_no);
```

### Schema notes

- `object_id` represents a graph edge target; `value_json` represents a literal/object payload. Application validation should require exactly one of these for each predicate class; SQLite's simple check above only enforces at least one.
- Add a schema migration/version table before shipping migrations.
- JSON payloads require size limits and validation at the application boundary.
- Avoid relying on SQLite `rowid` as a domain identifier.
- Add partial unique indexes only after defining the intended assertion identity and overlap semantics. Do not impose a uniqueness rule that accidentally forbids legitimate conflicting claims.

## Query contract

`query_as_known_at(t)` selects rows where `recorded_from <= t AND (recorded_to IS NULL OR t < recorded_to)`.

`query_valid_at(v, known_at)` applies both the transaction-time predicate above and `valid_from <= v AND (valid_to IS NULL OR v < valid_to)`.

A “current knowledge, historically valid” query uses current transaction time plus the requested valid-time instant. These modes must be separate named APIs to prevent accidental time-axis conflation.

## Correction transaction

Within one write transaction:

1. Resolve and validate the source/provenance record.
2. Find the active transaction-time version(s) for the assertion identity.
3. Close superseded versions by setting `recorded_to = now`.
4. Insert the replacement version with `recorded_from = now` and `recorded_to = NULL`.
5. Update or enqueue rebuildable projections in the same transaction (or use a transactional outbox if projection work is asynchronous).
6. Commit; on any failure, roll back all canonical changes.

The assertion identity and conflict policy must be specified before step 2 is implemented. Concurrent writers need serialized transactions or optimistic version checks.

## Graph projection

The canonical fact table is authoritative. A graph adjacency/index is a derived projection keyed by `fact_id`; it must be rebuildable and must retain temporal and provenance fields. Traversal is bounded by configured maximum depth, visited-node count, and result count. Cycles must terminate via a visited set.

## Security and privacy

- Retrieved content is data, not executable policy.
- Tool authorization comes only from trusted policy/procedural configuration.
- `access_scope` is enforced before retrieval and graph expansion, not merely filtered in presentation.
- Retention/deletion must cover canonical rows, FTS indexes, graph projections, caches, and exported artifacts under the documented retention policy.
- Do not store inferred sensitive traits as a “digital genotype.” Any persistent preference profile requires explicit purpose, consent, inspectability, correction, export, and deletion semantics.

## Required tests before merge

1. Valid-time query returns only assertions whose half-open interval contains the requested instant.
2. Transaction-time query reconstructs what the system knew at a prior instant.
3. Correction preserves prior transaction-time history.
4. A failed correction rolls back closure and insertion together.
5. Duplicate event idempotency key returns the existing outcome without duplicating effects.
6. Conflicting `(stream_id, sequence_no)` is rejected deterministically.
7. Restart/reopen preserves facts and events.
8. Migration from empty database and previous schema succeeds; migration failure leaves prior schema usable.
9. Graph traversal terminates on cycles and obeys depth/node/result caps.
10. Access-scope denial is enforced on direct lookup and graph expansion.
11. Deletion/retention removes or invalidates derived projections as specified.

## Acceptance gate

Do not mark Stage 2 complete until the schema is implemented in the Rust crate, migrations execute, all tests above have automated coverage, and CI passes. Record the exact commit and CI run in `POSTĘP.md`.