use std::sync::Arc;

use crate::application::interface::editing_session_repository::error::EditingSessionRepositoryError;
use crate::application::interface::editing_session_repository::repository::EditingSessionRepository;
use crate::application::types::editing_session::session::CommonEditingSession;
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::repository::editing_session_repository::SAM2EditingSessionRepository;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

#[derive(Clone)]
pub struct SharedEditingSessionRepository {
    sessions: Arc<SAM2EditingSessionRepository>,
}

impl SharedEditingSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(SAM2EditingSessionRepository::new()),
        }
    }
}

impl EditingSessionRepository for SharedEditingSessionRepository {
    type InferenceContext = SAM2InferenceContext;
    type StaticContext = SAM2StaticContext;

    fn get(
        &self,
        session_id: &SessionId,
    ) -> Result<CommonEditingSession<Self::StaticContext, Self::InferenceContext>, EditingSessionRepositoryError> {
        self.sessions.get(session_id)
    }

    fn save(
        &self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    ) {
        self.sessions.save(session_id, editing_session);
    }
}
