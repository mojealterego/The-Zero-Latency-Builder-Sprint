//! Small in-memory replay ledger. It records decisions/results only; it never
//! invokes tools or repeats external side effects.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub event_id: String,
    pub idempotency_key: String,
    pub action: String,
    pub policy_version: String,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
    EmptyEventId,
    EmptyIdempotencyKey,
    DuplicateEventId,
    IdempotencyConflict,
}

#[derive(Debug, Default)]
pub struct ReplayLedger {
    events: Vec<AuditEvent>,
    event_ids: BTreeMap<String, usize>,
    idempotency: BTreeMap<String, String>,
}

impl ReplayLedger {
    pub fn record(&mut self, event: AuditEvent) -> Result<(), RecordError> {
        if event.event_id.trim().is_empty() {
            return Err(RecordError::EmptyEventId);
        }
        if event.idempotency_key.trim().is_empty() {
            return Err(RecordError::EmptyIdempotencyKey);
        }
        if self.event_ids.contains_key(&event.event_id) {
            return Err(RecordError::DuplicateEventId);
        }
        if let Some(existing_event_id) = self.idempotency.get(&event.idempotency_key) {
            if existing_event_id != &event.event_id {
                return Err(RecordError::IdempotencyConflict);
            }
        }
        let index = self.events.len();
        self.event_ids.insert(event.event_id.clone(), index);
        self.idempotency.insert(event.idempotency_key.clone(), event.event_id.clone());
        self.events.push(event);
        Ok(())
    }

    /// Read-only replay: returns a cloned audit sequence and performs no actions.
    pub fn replay(&self) -> Vec<AuditEvent> {
        self.events.clone()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(id: &str, key: &str) -> AuditEvent {
        AuditEvent {
            event_id: id.into(),
            idempotency_key: key.into(),
            action: "notes.search".into(),
            policy_version: "v1".into(),
            outcome: "dry_run".into(),
        }
    }

    #[test]
    fn replay_preserves_recorded_order_without_executing_actions() {
        let mut ledger = ReplayLedger::default();
        ledger.record(event("e1", "k1")).unwrap();
        ledger.record(event("e2", "k2")).unwrap();
        assert_eq!(ledger.replay().iter().map(|e| e.event_id.as_str()).collect::<Vec<_>>(), vec!["e1", "e2"]);
    }

    #[test]
    fn rejects_duplicate_event_ids() {
        let mut ledger = ReplayLedger::default();
        ledger.record(event("e1", "k1")).unwrap();
        assert_eq!(ledger.record(event("e1", "k2")), Err(RecordError::DuplicateEventId));
    }

    #[test]
    fn rejects_idempotency_key_reuse_for_another_event() {
        let mut ledger = ReplayLedger::default();
        ledger.record(event("e1", "same-key")).unwrap();
        assert_eq!(ledger.record(event("e2", "same-key")), Err(RecordError::IdempotencyConflict));
    }

    #[test]
    fn rejects_empty_identifiers() {
        let mut ledger = ReplayLedger::default();
        assert_eq!(ledger.record(event(" ", "k")), Err(RecordError::EmptyEventId));
        assert_eq!(ledger.record(event("e", " ")), Err(RecordError::EmptyIdempotencyKey));
    }
}
