use chrono::Duration;
use serde::{Serialize, Deserialize};
use crate::models::timesheet::Timesheet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeTrackingReport {
    pub user_id: u32,
    pub timesheets: Vec<Timesheet>,
}

impl TimeTrackingReport {
    pub fn new(user_id: u32) -> Self {
        Self {
            user_id,
            timesheets: vec![],
        }
    }

    pub fn add_timesheet(&mut self, timesheet: Timesheet) {
        self.timesheets.push(timesheet);
    }

    pub fn generate_report(&self) -> String {
        let total_time: Duration = self.timesheets.iter().map(|sheet| sheet.total_time_spent()).sum();

        format!(
            "Time Tracking Report for User ID {}\nTotal Time Spent: {} hours\n",
            self.user_id,
            total_time.num_hours()
        )
    }
}