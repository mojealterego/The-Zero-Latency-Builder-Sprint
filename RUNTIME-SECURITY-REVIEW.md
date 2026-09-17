# Runtime Security Review

**Status:** Source inspection only; findings below are not a substitute for executed tests.

## Scope

Reviewed the B2B runtime files in `mojealterego/Nowe-projekty`:

- `agents/b2b/runtime/audit.ts`
- `agents/b2b/runtime/retry.ts`
- `agents/b2b/runtime/registry.ts`
- `agents/b2b/runtime/types.ts`

This is a bounded review of the files listed above, not an audit of every project or repository.

## Findings

### R1 — Retry can duplicate side effects after timeout

`withTimeout()` races the operation against a timer; it does not cancel the underlying operation. `retry()` may start another attempt after a timeout even if the previous attempt is still running or has already performed an external side effect.

**Risk:** duplicate sends, writes, charges, or other non-idempotent effects when a tool reports late or times out ambiguously.

**Required mitigation:** distinguish retryable failures from ambiguous outcomes; require downstream idempotency keys for side-effecting tools; support cancellation where the adapter can guarantee it; use operation deadlines and reconciliation/readback before retrying ambiguous executions.

### R2 — Retry options are not validated here

The retry helper accepts caller-supplied retry count, base delay, and timeout without explicit range validation.

**Risk:** invalid or extreme configuration can create immediate failures, excessive delays, or unexpectedly large retry counts.

**Required mitigation:** validate integer retry bounds and finite, non-negative delay/timeout values; cap maximum attempts and total elapsed time.

### R3 — Audit storage is process-local and mutable through returned references

`AuditStore` keeps events in an in-memory array. `Object.freeze(event)` is shallow, and `list()` returns an array containing the stored event references. TypeScript `readonly` does not provide runtime immutability.

**Risk:** loss on restart, lack of cross-instance consistency, and possible mutation of event fields/arrays by consumers.

**Required mitigation:** persist append-only audit records; return defensive copies or deeply immutable serialized records; include execution outcome, policy reasons, approval reference, attempt number, and evidence provenance; define retention and integrity verification.

### R4 — Audit event schema omits decision rationale and execution outcome

The reviewed `AuditEvent` records the decision and context identifiers, but has no explicit fields for policy reasons, final tool result, timeout/ambiguous outcome, or approval consumption.

**Risk:** incomplete forensic reconstruction and weak replay/evaluation evidence.

**Required mitigation:** version the event schema and record decision rationale, execution lifecycle transitions, normalized outcome/error, approval reference, and readback evidence without storing secrets.

### R5 — Registry does not validate runtime inputs or authorization

`ToolRegistry` rejects duplicate and unknown names, but the shown implementation does not validate tool input schemas, tenant/actor authorization, or policy compatibility. Generic input/output types are erased in the stored map.

**Risk:** malformed runtime payloads can reach tool implementations unless validation is guaranteed elsewhere.

**Required mitigation:** require each registered tool to provide a runtime input validator/schema and metadata; validate immediately before execution; keep authorization in the policy boundary and ensure the validated payload is the exact payload executed.

### R6 — TypeScript interfaces are not runtime trust boundaries

`ToolCall`, `ToolResult`, `AgentContext`, `ApprovalRequest`, and `AuditEvent` are compile-time interfaces. They do not validate untrusted JSON or values passed from adapters at runtime.

**Risk:** malformed identifiers, costs, resource strings, decisions, and nested output can bypass assumptions if callers/adapters are not validated.

**Required mitigation:** parse external inputs through runtime schemas; reject unknown or invalid fields where appropriate; enforce finite non-negative cost, bounded strings, allowed enums, and tenant/context consistency.

## Integration with the previously reviewed orchestrator

The earlier orchestrator inspection identified additional cross-file concerns that this review does not re-implement:

- Idempotency cache lookup precedes policy evaluation, so a cache hit may return without rechecking current authorization.
- The cache key is tenant plus caller-provided key and is not visibly bound to actor, tool, resource, or canonical input hash.
- Approval is not visibly bound to a canonical payload hash; the approval store's expiry handling for already-approved requests requires correction.
- In-memory idempotency and approval stores do not provide durable or atomic multi-instance behavior.

These are source-inspection findings from the previously reviewed files and should be validated with regression tests before remediation is considered complete.

## Recommended remediation order

1. **Execution safety:** bind idempotency to tenant + actor + tool + resource + canonical request hash; reject key reuse with a different request.
2. **Approval integrity:** bind approval to the exact request hash, policy version, actor, tool, resource, and expiry; enforce expiry at consumption; use atomic state transitions.
3. **Retry semantics:** classify retryable vs ambiguous outcomes; use downstream idempotency and readback/reconciliation before replaying side effects.
4. **Runtime validation:** validate context, tool call, policy response, and tool result at boundaries.
5. **Audit durability:** append-only persistent event log with immutable serialized records and lifecycle/outcome evidence.
6. **Concurrency and persistence:** use transactional/atomic stores for idempotency and approval state; define recovery behavior after process restart.

## Acceptance tests required

- A timed-out side-effecting operation is not blindly executed again; ambiguous outcome triggers reconciliation or a safe idempotent replay.
- Invalid retry options are rejected; maximum attempts and total deadline are bounded.
- Audit records survive process restart and cannot be mutated through returned values.
- Audit captures policy rationale, approval lifecycle, each execution attempt, final outcome, and readback evidence.
- Invalid tool input is rejected before tool execution.
- Reusing an idempotency key with a different actor, tool, resource, or payload is rejected.
- A cached result is not returned to a caller who no longer satisfies the applicable policy.
- Approval cannot be consumed after expiry or against a changed request payload/policy version.
- Concurrent duplicate requests result in at most one effective side effect.
- Replay tests cover timeout-after-side-effect, process restart, concurrent calls, approval expiry, and policy-version changes.

## Evidence status

- **Source inspected:** the four files listed in Scope.
- **Tests executed:** none in this review.
- **Benchmarks executed:** none.
- **Moss integration verified:** no.
- **Production readiness established:** no.
