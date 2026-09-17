//! Connects policy decisions to the in-memory audit ledger.
//! This module deliberately does not execute external tools.

use crate::policy::{authorize, Decision, ExecutionMode};
use crate::replay::{AuditEvent, RecordError, ReplayLedger};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRequest<'a> {
    pub event_id: &'a str,
    pub idempotency_key: &'a str,
    pub action: &'a str,
    pub policy_version: &'a str,
    pub allowed_actions: &'a [&'a str],
    pub requires_approval: bool,
    pub approval_granted: bool,
    pub mode: ExecutionMode,
}

/// Authorize and record the decision. No tool execution or side effect occurs.
pub fn decide_and_record(
    ledger: &mut ReplayLedger,
    request: DecisionRequest<'_>,
) -> Result<Decision, RecordError> {
    let decision = authorize(
        request.action,
        request.allowed_actions,
        request.requires_approval,
        request.approval_granted,
        request.mode,
    );
    let outcome = match decision {
        Decision::DenyUnknownAction => "deny_unknown_action",
        Decision::DenyApprovalRequired => "deny_approval_required",
        Decision::AllowDryRun => "allow_dry_run",
        Decision::AllowLive => "allow_live",
    };
    ledger.record(AuditEvent {
        event_id: request.event_id.to_string(),
        idempotency_key: request.idempotency_key.to_string(),
        action: request.action.to_string(),
        policy_version: request.policy_version.to_string(),
        outcome: outcome.to_string(),
    })?;
    Ok(decision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denied_request_is_audited_and_never_executed() {
        let mut ledger = ReplayLedger::default();
        let decision = decide_and_record(
            &mut ledger,
            DecisionRequest {
                event_id: "evt-1",
                idempotency_key: "idem-1",
                action: "messages.send",
                policy_version: "test-v1",
                allowed_actions: &["notes.search"],
                requires_approval: true,
                approval_granted: false,
                mode: ExecutionMode::Live,
            },
        )
        .unwrap();

        assert_eq!(decision, Decision::DenyUnknownAction);
        let events = ledger.replay();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].outcome, "deny_unknown_action");
    }

    #[test]
    fn approved_allowlisted_live_request_is_recorded() {
        let mut ledger = ReplayLedger::default();
        let decision = decide_and_record(
            &mut ledger,
            DecisionRequest {
                event_id: "evt-2",
                idempotency_key: "idem-2",
                action: "messages.send",
                policy_version: "test-v1",
                allowed_actions: &["messages.send"],
                requires_approval: true,
                approval_granted: true,
                mode: ExecutionMode::Live,
            },
        )
        .unwrap();
        assert_eq!(decision, Decision::AllowLive);
        assert_eq!(ledger.replay()[0].outcome, "allow_live");
    }
}
