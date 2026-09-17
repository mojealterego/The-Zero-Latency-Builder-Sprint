# Zero-Latency Builder Sprint

> **Status: Architecture proposal — performance claims are unverified.** This document defines a measurable implementation plan. It does not claim that sub-10 ms end-to-end agent responses have been achieved.

## Executive Summary

A local-first context engine for real-time agents, built around deterministic retrieval, bounded state transitions, and an auditable execution loop. The design separates the latency-critical local path from model inference, network calls, and durable background work.

## Hard Latency Boundary

“Sub-10 ms” is defined here as **local context selection latency**, measured from receipt of a normalized query to emission of a ranked context bundle, on a declared reference machine with a warm index. It does **not** include speech recognition, remote model inference, tool execution, network transport, or speech synthesis. End-to-end voice response latency is reported separately.

No performance number is accepted without: hardware/OS disclosure, dataset size and shape, warm/cold distinction, concurrency, p50/p95/p99, sample count, and reproducible benchmark command.

## Problem

Conventional RAG pipelines often put embedding generation, vector-store access, reranking, remote orchestration, and model inference in series. These stages can create variable latency and complicate offline operation. The problem is not that vector databases are inherently slow; it is that unnecessary work and remote dependencies may sit on the critical path.

## Proposed Solution

Use a local-first retrieval core with:

1. Exact identifiers and lexical matching for deterministic fast-path hits.
2. A compact inverted index with BM25-style scoring for text retrieval.
3. Optional precomputed concept aliases and metadata filters; no vector database in the MVP.
4. Bounded top-k retrieval and a strict context-token budget.
5. A local event log for agent state and replayable transitions.
6. Moss integration behind a narrow adapter, after its actual SDK/API contract is verified.
7. Remote LLM, ASR/TTS, and external tools as asynchronous/non-critical dependencies wherever product semantics permit.

## Four Track Mapping

| Track | MVP contribution | Measurement |
|---|---|---|
| Real-Time Voice | Streaming turn state, interruptible output contract, local context lookup | Local retrieval p95; separately report ASR/LLM/TTS and end-to-end latency |
| Multiplayer AI | Shared append-only events, versioned room state, deterministic conflict policy | State convergence, event propagation p50/p95, duplicate handling |
| Local-First AI | On-device index, local persistence, offline query path | Offline success rate, cold/warm latency, memory and disk footprint |
| Agent Reliability | Explicit state machine, idempotency keys, bounded retries, trace IDs, replay | Task completion, recovery rate, duplicate side effects, trace completeness |

## Clean Architecture

```text
Presentation / CLI / Voice UI
          |
Application: QueryContext, IngestDocument, RunAgentTurn
          |
Domain: Document, Query, ContextItem, AgentEvent, Policy
          |
Ports: SearchIndex, EventStore, MossRuntime, Clock, Telemetry
          |
Adapters: SQLite + FTS5, in-memory test doubles, Moss SDK adapter
```

Dependencies point inward. Domain and application layers must not import database, UI, networking, or Moss SDK modules.

## Retrieval Data Model (No Vector Store)

SQLite is the proposed durable store; FTS5 provides lexical full-text indexing. The authoritative content remains in normalized tables, while FTS is a rebuildable index.

```sql
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS documents (
  id TEXT PRIMARY KEY,
  source TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  content_hash TEXT NOT NULL UNIQUE,
  updated_at INTEGER NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(
  title, body, content='documents', content_rowid='rowid',
  tokenize='unicode61 remove_diacritics 2'
);

CREATE TABLE IF NOT EXISTS agent_events (
  event_id TEXT PRIMARY KEY,
  room_id TEXT NOT NULL,
  sequence_no INTEGER NOT NULL,
  actor_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  UNIQUE(room_id, sequence_no)
);

CREATE INDEX IF NOT EXISTS idx_events_room_sequence
  ON agent_events(room_id, sequence_no);
```

SQLite FTS5 is a lexical retrieval mechanism, not semantic understanding. Semantic-like matching in the MVP is limited to curated aliases, entity metadata, and query expansion; quality must be evaluated against a labeled query set. A learned embedding model is intentionally excluded from the latency-critical path.

## Agent State Machine

```text
IDLE -> RECEIVING -> CONTEXT_READY -> PLANNING -> EXECUTING
  ^           |             |             |             |
  |           +-> CANCELLED  +-> FAILED    +-> FAILED    +-> VERIFYING
  |                                                         |
  +-------------------- COMPLETED <-------------------------+
```

Every transition carries `turn_id`, `trace_id`, `state_version`, and an idempotency key. Retries are bounded and apply only to explicitly retryable operations. Tool calls with external side effects require idempotency support or a reconciliation step. Cancellation is cooperative and checked between stages.

## Moss Integration Boundary

`MossRuntime` is a port, not a guessed SDK wrapper. Before implementation, pin the official package/version, authentication model, runtime lifecycle, streaming semantics, cancellation behavior, and supported deployment targets. Keep Moss-specific types inside `adapters/moss/`. Until those facts are verified, the application can be tested against a deterministic in-memory runtime adapter.

## Repository Layout

```text
src/
  domain/        # Pure entities, value objects, policies
  application/   # Use cases and orchestration
  ports/         # Interfaces for storage, runtime, telemetry
  adapters/
    sqlite/      # Durable data and FTS5 index
    moss/        # Verified Moss integration
    telemetry/   # Metrics and tracing
  entrypoints/   # CLI/API/voice transport
 tests/
  unit/
  integration/
  benchmarks/
  retrieval_dataset.jsonl
```

## Benchmark Protocol

See [`BENCHMARK_PROTOCOL.md`](BENCHMARK_PROTOCOL.md) for the reproducible commands, required metadata, and interpretation constraints.

- Freeze a representative corpus and labeled query set; publish corpus size, language distribution, and query classes.
- Measure index build time, resident memory, ingest throughput, and query p50/p95/p99.
- Separate exact-hit, lexical, alias-expanded, and no-hit cases.
- Run warm and cold tests; report CPU model, RAM, OS, runtime/compiler versions, and concurrency.
- Use a monotonic clock; avoid logging or serialization inside the timed region.
- Compare retrieval quality using Recall@k and MRR against the labeled set.
- Treat the 10 ms target as a release gate for local retrieval only. Never conflate it with LLM response time.

## Reliability and Security

- Validate and size-limit all ingested documents and event payloads.
- Treat retrieved text as untrusted data, never as executable instructions.
- Keep secrets out of the repository and logs.
- Use explicit allowlists for tools and capabilities.
- Persist event sequence numbers transactionally; reject conflicting duplicates.
- Provide replay and deterministic test doubles for failure scenarios.

## Implementation Gates

1. Verify hackathon rules, judging rubric, official Moss documentation, and SDK access.
2. Inventory source repositories and license/provenance before copying any code.
3. Implement and test the lexical local retrieval vertical slice.
4. Establish reproducible latency and retrieval-quality baselines.
5. Add Moss adapter and demonstrate a real-time workflow.
6. Add collaboration and reliability demonstrations only after core measurements pass.

## Grant / Devpost Pitch (Draft)

### The Problem

Agent applications frequently chain retrieval, orchestration, model inference, and tools into one serial path. Remote dependencies and heavyweight retrieval stages can make interactive workflows unpredictable, while cloud-only assumptions undermine privacy and offline resilience.

### The Solution

We are building a local-first context engine that makes context selection a bounded, measurable operation. Exact lookup, lexical indexing, curated aliases, and metadata filtering run locally; model inference and external actions are isolated behind explicit adapters. The MVP avoids vector databases and keeps a replayable event log for agent state.

### Technical Architecture

Clean Architecture separates domain policy from storage and runtime integrations. SQLite/FTS5 provides a compact, rebuildable lexical index; a versioned event log supports replay and collaborative state. Moss is integrated through a dedicated adapter once its official runtime contract is verified. We report local retrieval latency separately from network and model latency, with p50/p95/p99 and reproducible workloads.

### Market Impact and Scalability

The architecture targets developers building voice interfaces, collaborative AI experiences, and private/offline assistants. The core can scale from a single-device prototype to synchronized rooms by separating local state, append-only events, and remote coordination. The differentiator is not an unsupported “zero-latency” claim: it is a transparent latency budget, offline-capable retrieval, and reliability evidence that teams can reproduce.

## Current Evidence Status

- Rust crate now has a deterministic lexical selector, policy gate, in-memory replay ledger, runtime module, and synthetic retrieval benchmark.
- `src/lib.rs` formatting and test layout were normalized in the latest source commit; CI is responsible for validating the change.
- The benchmark protocol is documented in `BENCHMARK_PROTOCOL.md`.
- A previous local synthetic run reported by the user is not evidence of production or end-to-end latency.
- No Moss SDK integration is claimed until the official API contract is verified.
