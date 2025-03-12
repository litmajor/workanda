use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AvailabilityStatus {
    Available,
    Unavailable,
}