use std::collections::HashMap;

use crate::application::interface::editing_session_repository::EditingSessionRepository;
use crate::application::types::editing_session::{CommonEditingSession, EditingSession};
use crate::domain::value_object::session_id::SessionId;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

pub struct SAM2EditingSessionRepository{
    sessions: HashMap<SessionId, CommonEditingSession<SAM2StaticContext, SAM2InferenceContext>>
}

impl EditingSessionRepository for SAM2EditingSessionRepository {
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;

    fn save(
        &mut self,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    )
    {
        let session_id = editing_session.session().session_id();
        self.sessions.insert(*session_id, editing_session);
    }

    fn get(
        &self,
        session_id: &SessionId,
    ) -> Option<&CommonEditingSession<Self::StaticContext, Self::InferenceContext>>
    {
        self.sessions.get(session_id)
    }
}
