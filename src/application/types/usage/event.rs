use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::{application::types::usage::{operation::UsageOperation, status::UsageStatus}, domain::value_object::image_size::image_size::ImageSize};

pub struct UsageEvent {
    operation: UsageOperation,
    status: UsageStatus,
    processing_time: Duration,
    image_size: Option<ImageSize>,
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
    
    pub fn image_size(&self) -> Option<&ImageSize> {
        self.image_size.as_ref()
    }
    
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}