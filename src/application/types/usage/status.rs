use crate::application::error::ApplicationError;

pub enum UsageStatus {
    Success,
    Failed(ApplicationError),
}