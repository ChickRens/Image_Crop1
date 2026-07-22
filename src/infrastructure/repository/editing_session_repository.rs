use std::collections::HashMap;
use std::sync::Mutex;

use crate::application::interface::editing_session_repository::EditingSessionRepository;
use crate::application::types::editing_session::CommonEditingSession;
use crate::domain::value_object::session_id::SessionId;
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
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(*session_id, editing_session);
    }

    fn get(
        &self,
        session_id: &SessionId,
    ) -> Option<CommonEditingSession<Self::StaticContext, Self::InferenceContext>> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(session_id)
    }
}

impl SAM2EditingSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}
