use std::time::Duration;

use crate::application::types::usage::{operation::UsageOperation, status::UsageStatus};

pub struct UsageEvent {
    operation: UsageOperation,
    status: UsageStatus,
    processing_time: Duration,
}

impl UsageEvent {
    pub fn new(operation: UsageOperation, status: UsageStatus, processing_time: Duration) -> Self {
        Self { operation, status, processing_time }
    }
    
    pub fn operation(&self) -> &UsageOperation {
        &self.operation
    }
    
    pub fn status(&self) -> &UsageStatus {
        &self.status
    }
    
    pub fn processing_time(&self) -> Duration {
        self.processing_time
    }
}