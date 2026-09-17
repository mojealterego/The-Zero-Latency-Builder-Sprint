# Runtime Tool Policy Contract

Status: specification only; runtime enforcement is not implemented yet.

## Request envelope

A tool request must include `run_id`, `request_id`, `tool_name`, `operation`, canonical `arguments`, `arguments_hash`, `requested_by`, and an explicit `side_effect_class` (`read_only`, `reversible_write`, or `irreversible_write`). Reject unknown fields unless the tool schema explicitly allows them.

## Decision states

- `allow`: read-only operation matches a registered schema and policy.
- `require_approval`: write operation is eligible but needs a valid approval bound to the exact request hash.
- `deny`: schema mismatch, unknown tool/operation, invalid arguments, expired approval, policy violation, or missing required identity.

No implicit allow on parse errors, missing policy, timeout, or unavailable approval service.

## Enforcement sequence

1. Resolve tool from an allowlisted registry; never dispatch by arbitrary name or namespace.
2. Validate arguments against the registered schema and reject unknown keys.
3. Canonicalize arguments and calculate a cryptographic request hash.
4. Evaluate policy using actor, tool, operation, side-effect class, and request hash.
5. For writes, validate one-time approval bound to the same hash, actor, operation, expiry, and nonce.
6. Check idempotency key before execution; persist execution intent before external side effect.
7. Record outcome, redacting secrets and sensitive values.

## Failure behavior

- Fail closed if policy evaluation or approval validation fails.
- Do not automatically retry a side-effecting operation unless the tool declares idempotency semantics and the same idempotency key is reused.
- A timeout means outcome is unknown unless the tool can confirm cancellation or query status. Do not report it as definitely failed.
- Separate retrying a read-only lookup from repeating a write.

## Required tests

- Unknown tool and unknown operation are denied.
- Schema-invalid and extra arguments are denied.
- Approval for one request hash cannot authorize a changed request.
- Expired/reused approval is denied.
- Timeout does not trigger unsafe duplicate write.
- Audit record is emitted for allow, approval-required, deny, and execution outcomes.

## Security boundary

This contract is not a security certification. Production use requires implementation, threat modeling, tests, and review of the actual executor and persistence layer.