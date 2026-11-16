// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-fan-control: fan control component model.
//!
//! Real implementation: controls, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Control {
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
pub enum FanControlError {
    EmptyControlName,
    DuplicateControl(String),
    EmptyEventSubject,
}

impl fmt::Display for FanControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyControlName => write!(f, "control name must not be empty"),
            Self::DuplicateControl(id) => write!(f, "duplicate control {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for FanControlError {}

pub struct FanControl {
    pub name: String,
    controls: BTreeMap<String, Control>,
    events: BTreeMap<String, Event>,
}

impl FanControl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            controls: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_control(&mut self, item: Control) -> Result<(), FanControlError> {
        if item.name.trim().is_empty() {
            return Err(FanControlError::EmptyControlName);
        }
        let id = item.id.clone();
        if self.controls.contains_key(&id) {
            return Err(FanControlError::DuplicateControl(id.clone()));
        }
        self.controls.insert(id, item);
        Ok(())
    }

    pub fn control_count(&self) -> usize {
        self.controls.len()
    }

    pub fn get_control(&self, id: &str) -> Option<&Control> {
        self.controls.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), FanControlError> {
        if event.subject.trim().is_empty() {
            return Err(FanControlError::EmptyEventSubject);
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

    fn sample_control(id: &str, name: &str) -> Control {
        Control {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = FanControl::new("FanControl");
        assert_eq!(m.name, "FanControl");
        assert_eq!(m.control_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_control() {
        let mut m = FanControl::new("FanControl");
        m.add_control(sample_control("a1", "Alpha")).unwrap();
        assert_eq!(m.control_count(), 1);
        assert!(m.get_control("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = FanControl::new("FanControl");
        assert!(m.add_control(sample_control("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = FanControl::new("FanControl");
        m.add_control(sample_control("x", "One")).unwrap();
        assert!(m.add_control(sample_control("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = FanControl::new("FanControl");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
