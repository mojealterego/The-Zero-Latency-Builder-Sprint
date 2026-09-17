//! Minimal fail-closed policy gate for tool execution.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    DryRun,
    Live,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    DenyUnknownAction,
    DenyApprovalRequired,
    AllowDryRun,
    AllowLive,
}

/// Decide whether an action may execute. `allowed_actions` must be a trusted
/// configuration supplied by the application, not by the model or user input.
pub fn authorize(
    action: &str,
    allowed_actions: &[&str],
    requires_approval: bool,
    approval_granted: bool,
    mode: ExecutionMode,
) -> Decision {
    // Reject blank identifiers before consulting configuration. Exact matching
    // avoids namespace/prefix bypasses such as `x.send`.
    if action.trim().is_empty()
        || !allowed_actions
            .iter()
            .any(|allowed| !allowed.trim().is_empty() && *allowed == action)
    {
        return Decision::DenyUnknownAction;
    }
    if mode == ExecutionMode::DryRun {
        return Decision::AllowDryRun;
    }
    if requires_approval && !approval_granted {
        return Decision::DenyApprovalRequired;
    }
    Decision::AllowLive
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOW: &[&str] = &["notes.search", "messages.send"];

    #[test]
    fn exact_allowlist_only() {
        assert_eq!(
            authorize("messages.send", ALLOW, false, false, ExecutionMode::Live),
            Decision::AllowLive
        );
        assert_eq!(
            authorize(
                "admin.messages.send",
                ALLOW,
                false,
                true,
                ExecutionMode::Live
            ),
            Decision::DenyUnknownAction
        );
    }

    #[test]
    fn rejects_blank_action_even_if_allowlist_contains_blank() {
        assert_eq!(
            authorize(" ", &[" "], false, true, ExecutionMode::Live),
            Decision::DenyUnknownAction
        );
        assert_eq!(
            authorize("", ALLOW, false, true, ExecutionMode::Live),
            Decision::DenyUnknownAction
        );
    }

    #[test]
    fn live_external_action_requires_approval() {
        assert_eq!(
            authorize("messages.send", ALLOW, true, false, ExecutionMode::Live),
            Decision::DenyApprovalRequired
        );
        assert_eq!(
            authorize("messages.send", ALLOW, true, true, ExecutionMode::Live),
            Decision::AllowLive
        );
    }

    #[test]
    fn dry_run_never_requires_live_approval() {
        assert_eq!(
            authorize("messages.send", ALLOW, true, false, ExecutionMode::DryRun),
            Decision::AllowDryRun
        );
    }
}
