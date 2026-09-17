-- Stage 2 foundation: canonical, append-oriented bitemporal memory.
-- SQLite-compatible. Apply with foreign_keys=ON.
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS memory_sources (
    source_id TEXT PRIMARY KEY NOT NULL,
    source_kind TEXT NOT NULL,
    locator TEXT,
    content_hash TEXT,
    recorded_at TEXT NOT NULL,
    access_scope TEXT NOT NULL DEFAULT 'private',
    CHECK (length(source_id) > 0),
    CHECK (length(source_kind) > 0),
    CHECK (length(access_scope) > 0)
);

CREATE TABLE IF NOT EXISTS memory_facts (
    fact_id TEXT PRIMARY KEY NOT NULL,
    subject_id TEXT NOT NULL,
    predicate TEXT NOT NULL,
    object_id TEXT,
    value_json TEXT,
    valid_from TEXT NOT NULL,
    valid_to TEXT,
    recorded_at TEXT NOT NULL,
    superseded_at TEXT,
    source_id TEXT NOT NULL REFERENCES memory_sources(source_id),
    confidence REAL NOT NULL DEFAULT 1.0 CHECK (confidence >= 0.0 AND confidence <= 1.0),
    access_scope TEXT NOT NULL DEFAULT 'private',
    content_hash TEXT,
    CHECK (length(subject_id) > 0),
    CHECK (length(predicate) > 0),
    CHECK (object_id IS NOT NULL OR value_json IS NOT NULL),
    CHECK (valid_to IS NULL OR valid_to >= valid_from),
    CHECK (superseded_at IS NULL OR superseded_at >= recorded_at)
);

CREATE INDEX IF NOT EXISTS idx_memory_facts_subject_predicate
    ON memory_facts(subject_id, predicate);
CREATE INDEX IF NOT EXISTS idx_memory_facts_valid_time
    ON memory_facts(valid_from, valid_to);
CREATE INDEX IF NOT EXISTS idx_memory_facts_transaction_time
    ON memory_facts(recorded_at, superseded_at);
CREATE INDEX IF NOT EXISTS idx_memory_facts_source
    ON memory_facts(source_id);

CREATE TABLE IF NOT EXISTS memory_events (
    event_id TEXT PRIMARY KEY NOT NULL,
    idempotency_key TEXT NOT NULL UNIQUE,
    actor TEXT NOT NULL,
    event_kind TEXT NOT NULL,
    occurred_at TEXT NOT NULL,
    recorded_at TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    source_id TEXT REFERENCES memory_sources(source_id),
    CHECK (length(actor) > 0),
    CHECK (length(event_kind) > 0)
);

-- Append-only guardrails: corrections should supersede, not erase, prior versions.
CREATE TRIGGER IF NOT EXISTS memory_facts_no_delete
BEFORE DELETE ON memory_facts
BEGIN
    SELECT RAISE(ABORT, 'memory_facts are append-oriented; use retention workflow');
END;

CREATE TRIGGER IF NOT EXISTS memory_events_no_update
BEFORE UPDATE ON memory_events
BEGIN
    SELECT RAISE(ABORT, 'memory_events are append-only');
END;

CREATE TRIGGER IF NOT EXISTS memory_events_no_delete
BEFORE DELETE ON memory_events
BEGIN
    SELECT RAISE(ABORT, 'memory_events are append-only');
END;

-- Current belief as of transaction time T: recorded_at <= T and not yet superseded at T.
-- Application must bind a normalized UTC timestamp as :transaction_time.
-- SELECT * FROM memory_facts
-- WHERE recorded_at <= :transaction_time
--   AND (superseded_at IS NULL OR superseded_at > :transaction_time);

-- Facts valid in the modeled world at valid time T, restricted to what was known at T.
-- Application must bind normalized UTC timestamps :valid_time and :transaction_time.
-- SELECT * FROM memory_facts
-- WHERE valid_from <= :valid_time
--   AND (valid_to IS NULL OR valid_to > :valid_time)
--   AND recorded_at <= :transaction_time
--   AND (superseded_at IS NULL OR superseded_at > :transaction_time);
