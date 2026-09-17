# Source Component Audit — Initial Findings

**Scope:** targeted source inspection of `mojealterego/Nowe-projekty`, focused on `agent-cashflow-collections` and the shared policy adapter/engine. This is not a full audit of the portfolio.

**Evidence status:** source files were read through GitHub. Tests were inspected but not executed in this environment. No benchmark or Moss integration was run or verified.

## 1. Candidate source slice

| Component | Observed implementation | Relevance to sprint | Status |
|---|---|---|---|
| `agent-cashflow-collections/src/domain.ts` | Deterministic invoice scoring from amount, overdue days, payment history, and disputes | Reproducible workload for latency/replay tests; not itself semantic retrieval | Source inspected |
| `agent-cashflow-collections/src/tools.ts` | Prioritization, message drafting, and a delivery boundary that returns a plan rather than contacting a provider | Useful consequential-action example for policy + assurance demonstrations | Source inspected |
| `agent-cashflow-collections/src/runtime.ts` | Registers tools with shared orchestrator and policy adapter | Potential integration seam | Source inspected; shared runtime implementation still needs review |
| `agents/b2b/runtime/policy-engine-adapter.ts` | Maps agent context and tool call metadata into `PolicyEngine.evaluate` | Candidate policy boundary | Source inspected |
| `agent-policy-gateway/src/policy.ts` | Rule-based allow/review/deny, risk/cost limits, explicit-policy check for selected destructive patterns | Candidate policy engine; requires security-focused tests and review | Source inspected |
| `agent-cashflow-collections/tests/runtime.test.ts` | Asserts deterministic prioritization, opt-out handling, approval, and replay behavior | Existing test intent can inform assurance regression suite | Inspected, not executed |

## 2. Concrete findings and risks

### F-01 — Invalid dates are not rejected before scoring

`overdueDays` computes from `new Date(dueDate).getTime()`. An invalid date can propagate `NaN` into the score; comparisons against thresholds then do not behave as intended. Add strict ISO/date validity checks and tests for malformed, empty, and boundary dates.

### F-02 — Invoice/customer identity consistency is not checked

The prioritization tool accepts an invoice and customer history independently. It does not verify `invoice.customerId === history.customerId`. Reject mismatched records before producing a decision; add a negative test.

### F-03 — Email validation is only a minimal shape check

The delivery boundary checks `recipient.includes("@")`. This is not sufficient validation for an email integration. Use a well-defined address validation strategy at the provider boundary and keep delivery disabled until that adapter exists.

### F-04 — Delivery is a plan, not external delivery

The `sendCollectionMessage` implementation returns `approved_for_delivery`; its comment explicitly says no external provider is contacted and that a real adapter must be injected after policy approval. Product copy and demos must not imply an email was sent.

### F-05 — Namespaced action names can evade the implicit destructive-pattern detector

The policy engine's destructive regex is anchored to the beginning of the action string (for example, `send_...`). A namespaced action such as `collections.send_message` does not match that regex. The current action has an explicit policy, but future namespaced side-effect actions need canonical action classification or a namespace-aware detector, plus tests proving unknown side-effect actions fail closed.

### F-06 — Validate policy request values at the boundary

The inspected policy engine compares `risk` and formats `estimatedCostEur`, but this file does not validate runtime input shape, finite/non-negative cost, or membership in the declared risk set. Add schema validation before evaluation; malformed values must deny safely rather than throw or bypass comparisons.

## 3. Proposed sprint vertical slice

Build a **local-first context selection + governed tool execution + evidence/replay** demonstration, not a claim of a complete multi-track product:

1. Local retrieval selects a bounded set of context records and records candidate IDs, scores, and elapsed time.
2. A canonical execution envelope carries tenant/actor/agent, correlation ID, action, resource, risk, estimated cost, and idempotency key.
3. Policy decision is recorded as allow/review/deny with policy version and reason codes.
4. Consequential actions remain non-executing until approval is validated; the demo's collection delivery remains a simulated plan.
5. Each run emits append-only evidence sufficient for deterministic replay and regression evaluation.
6. Benchmark local context selection independently from end-to-end model/tool latency; report p50/p95/p99, workload, hardware, warm/cold state, and retrieval quality.

## 4. Acceptance gates before making performance or reliability claims

- Unit tests for malformed input, tenant/customer mismatch, opt-out, unknown side-effect action, approval replay, idempotency collision, and policy-version changes.
- At least two materially different runtime adapters before claiming cross-framework assurance.
- Replay tests prove equivalent decisions and detect policy/configuration drift.
- Benchmark harness reports reproducible latency distributions and retrieval quality; no sub-10ms claim until measured under stated conditions.
- Any Moss integration is behind a verified adapter and documented with the exact API/version tested.
- README and pitch distinguish implemented, source-inspected, proposed, and measured capabilities.

## 5. Next source audit targets

- Shared B2B runtime: orchestrator, registry, types, approval store, idempotency behavior, audit/event persistence.
- `agent-compliance-evidence` and `agent-contract-obligations`: compare their schemas and postcondition/evidence handling against the candidate slice.
- `Knowledge-projects` Project 72 assurance baseline: map its canonical event, invariant, readback, replay, and drift requirements to executable code rather than prose.
