use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheck {
    pub status: String,
    pub timestamp: SystemTime,
}

impl HealthCheck {
    pub fn ok() -> HealthCheck {
        HealthCheck { status: "OK".to_string(), timestamp: SystemTime::now() }
    }
}