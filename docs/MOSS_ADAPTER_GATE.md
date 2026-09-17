# Moss Adapter Integration Gate

Status: integration blocked pending verification against current official Moss documentation/package/API. This file intentionally does not invent endpoints, package names, or method signatures.

## Adapter boundary

Keep application code independent of the Moss SDK behind a narrow interface:

- `index(source_id, content, metadata) -> Result<IndexReceipt>`
- `retrieve(query, limit, filters) -> Result<Vec<RetrievedItem>>`
- `health() -> Result<AdapterHealth>`

These are proposed internal Rust interface concepts, not claimed Moss API methods.

## Verification required before implementation

Record the official documentation URL, package/crate name, pinned version, supported runtime, authentication method, request/response schemas, error model, timeout/cancellation semantics, data retention behavior, and rate limits. Confirm whether the service supports local-first/offline operation and what data leaves the device.

## Integration acceptance criteria

1. A minimal official example compiles using the pinned dependency/version.
2. A fixture can be indexed and retrieved through the real adapter.
3. Results map to the internal `RetrievedItem` schema with source identity preserved.
4. Timeout, unavailable service, malformed response, and auth failure are tested.
5. Credentials are supplied through environment/secret configuration and never logged.
6. Benchmark reports adapter/network latency separately from local lexical baseline.
7. Retrieval quality is measured on a fixed, versioned query/ground-truth set; latency alone is insufficient.

## Fallback behavior

If Moss is unavailable, the app may use the existing lexical baseline only when the caller explicitly permits degraded mode. The response must identify the selected backend; it must not label lexical results as Moss semantic retrieval.

## Decision record template

- Verified on (UTC date):
- Official docs URL:
- Package and exact version:
- Runtime / deployment mode:
- Data locality and retention:
- Authentication and secret handling:
- API signatures verified by compiling example:
- Known limitations:
- Decision: integrate / defer, with rationale:
