// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-nsswitch: nsswitch component model.
//!
//! Real implementation: nsswitch_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NsswitchItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NsswitchError {
    EmptyNsswitchItemName,
    DuplicateNsswitchItem(String),
    EmptyEventSubject,
}

impl fmt::Display for NsswitchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNsswitchItemName => write!(f, "nsswitch_item name must not be empty"),
            Self::DuplicateNsswitchItem(id) => write!(f, "duplicate nsswitch_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NsswitchError {}

pub struct Nsswitch {
    pub name: String,
    nsswitch_items: BTreeMap<String, NsswitchItem>,
    events: BTreeMap<String, Event>,
}

impl Nsswitch {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nsswitch_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_nsswitch_item(&mut self, item: NsswitchItem) -> Result<(), NsswitchError> {
        if item.name.trim().is_empty() {
            return Err(NsswitchError::EmptyNsswitchItemName);
        }
        let id = item.id.clone();
        if self.nsswitch_items.contains_key(&id) {
            return Err(NsswitchError::DuplicateNsswitchItem(id.clone()));
        }
        self.nsswitch_items.insert(id, item);
        Ok(())
    }

    pub fn nsswitch_item_count(&self) -> usize {
        self.nsswitch_items.len()
    }

    pub fn get_nsswitch_item(&self, id: &str) -> Option<&NsswitchItem> {
        self.nsswitch_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NsswitchError> {
        if event.subject.trim().is_empty() {
            return Err(NsswitchError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nsswitch_item(id: &str, name: &str) -> NsswitchItem {
        NsswitchItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Nsswitch::new("Nsswitch");
        assert_eq!(m.name, "Nsswitch");
        assert_eq!(m.nsswitch_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_nsswitch_item() {
        let mut m = Nsswitch::new("Nsswitch");
        m.add_nsswitch_item(sample_nsswitch_item("a1", "Alpha")).unwrap();
        assert_eq!(m.nsswitch_item_count(), 1);
        assert!(m.get_nsswitch_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Nsswitch::new("Nsswitch");
        assert!(m.add_nsswitch_item(sample_nsswitch_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Nsswitch::new("Nsswitch");
        m.add_nsswitch_item(sample_nsswitch_item("x", "One")).unwrap();
        assert!(m.add_nsswitch_item(sample_nsswitch_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Nsswitch::new("Nsswitch");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
