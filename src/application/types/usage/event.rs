use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::application::types::usage::{operation::UsageOperation, status::UsageStatus};

pub struct UsageEvent {
    operation: UsageOperation,
    status: UsageStatus,
    processing_time: Duration,
    created_at: DateTime<Utc>
}

impl UsageEvent {
    pub fn operation(&self) -> &UsageOperation {
        &self.operation
    }
    
    pub fn status(&self) -> &UsageStatus {
        &self.status
    }
    
    pub fn processing_time(&self) -> Duration {
        self.processing_time
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}