use dashmap::DashMap;
use dashmap::mapref::one::RefMut;

use crate::application::interface::editing_session_repository::error::EditingSessionRepositoryError;
use crate::application::interface::editing_session_repository::repository::EditingSessionRepository;
use crate::application::types::editing_session::session::CommonEditingSession;
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

pub struct SAM2EditingSessionRepository {
    sessions: DashMap<SessionId, CommonEditingSession<SAM2StaticContext, SAM2InferenceContext>>,
}

impl EditingSessionRepository for SAM2EditingSessionRepository {
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;
    type Guard<'a>
        = RefMut<'a, SessionId, CommonEditingSession<Self::StaticContext, Self::InferenceContext>>
    where
        Self: 'a;

    fn save(
        &self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    ) {
        self.sessions.insert(*session_id, editing_session);
    }

    fn get<'a>(
        &'a self,
        session_id: &SessionId,
    ) -> Result<Self::Guard<'a>, EditingSessionRepositoryError> {
        self.sessions
            .get_mut(session_id)
            .ok_or(EditingSessionRepositoryError::EditingSessionNotFound)
    }
}

impl SAM2EditingSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }
}
