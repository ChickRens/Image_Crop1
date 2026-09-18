use sqlx::SqlitePool;
use tokio::sync::mpsc::{self, Sender};

use crate::{application::{interface::usage_recorder::UsageRecorder, types::usage::{event::UsageEvent, operation::UsageOperation, status::UsageStatus}}, common::traits::Code};

pub struct SQLiteUsageRecorder {
    sender: Sender<UsageEvent>
}

impl UsageRecorder for SQLiteUsageRecorder {
    async fn record(&self, event: UsageEvent) {
        if let Err(e) = self.sender.try_send(event) {
            eprintln!("{:?}",e);
        };
    }
}

impl SQLiteUsageRecorder {
    pub fn new(pool: SqlitePool) -> Self {
        let (tx, mut rx) = mpsc::channel::<UsageEvent>(1024);

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                let operation_str = match event.operation() {
                    UsageOperation::Upload => "upload".to_string(),
                    UsageOperation::Segment => "segment".to_string(),
                    UsageOperation::Save => "save".to_string(),
                    UsageOperation::Redo => "redo".to_string(),
                    UsageOperation::Undo => "undo".to_string(),
                    UsageOperation::GetPreview => "get-preview".to_string(),
                    UsageOperation::GetCompleted => "get-completed".to_string(),
                    UsageOperation::Unknown(err) => err.to_owned(),
                };

                let (status_str, failure_cause) = match event.status() {
                    UsageStatus::Failed(err) => ("failed".to_string(), Some(err.code().to_string())),
                    UsageStatus::Success => ("success".to_string(), None),
                };

                let processing_time_ms = event.processing_time().as_millis() as i64;

                let created_at = event.created_at().to_string();

                if let Err(e) = sqlx::query(
                    "INSERT INTO usage_records (operation, status, failure_cause, processing_time_ms, occurred_at) VALUE (?, ?, ?, ?, ?)"
                )
                .bind(operation_str)
                .bind(status_str)
                .bind(failure_cause)
                .bind(processing_time_ms)
                .bind(created_at)
                .execute(&pool)
                .await {
                    eprintln!("{}", e)
                }
            }
        });

        Self { sender: tx }
    }
}