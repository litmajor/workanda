use chrono::Duration;
use serde::{Serialize, Deserialize};
use crate::models::time_entry::TimeEntry;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timesheet {
    pub user_id: u32,
    pub time_entries: Vec<TimeEntry>,
}

impl Timesheet {
    pub fn new(user_id: u32) -> Self {
        Self {
            user_id,
            time_entries: vec![],
        }
    }

    pub fn add_time_entry(&mut self, time_entry: TimeEntry) {
        self.time_entries.push(time_entry);
    }

    pub fn total_time_spent(&self) -> Duration {
        self.time_entries.iter().map(|entry| entry.duration()).sum()
    }
}