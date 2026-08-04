use std::collections::HashMap;
use std::sync::Mutex;

use crate::application::interface::editing_session_repository::error::EditingSessionRepositoryError;
use crate::application::interface::editing_session_repository::repository::EditingSessionRepository;
use crate::application::types::editing_session::CommonEditingSession;
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

pub struct SAM2EditingSessionRepository {
    sessions:
        Mutex<HashMap<SessionId, CommonEditingSession<SAM2StaticContext, SAM2InferenceContext>>>,
}

impl EditingSessionRepository for SAM2EditingSessionRepository {
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;

    fn save(
        &self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    ) {
        let mut sessions = self
            .sessions
            .lock()
            .expect("EditingSessionRepository Mutex is Poisoned");
        sessions.insert(*session_id, editing_session);
    }

    fn get(
        &self,
        session_id: &SessionId,
    ) -> Result<CommonEditingSession<Self::StaticContext, Self::InferenceContext>, EditingSessionRepositoryError>
    {
        let mut sessions = self
            .sessions
            .lock()
            .expect("EditingSessionRepository Mutex is Poisoned");
        sessions.remove(session_id).ok_or(EditingSessionRepositoryError::EditingSessionNotFound)
    }
}

impl SAM2EditingSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}
