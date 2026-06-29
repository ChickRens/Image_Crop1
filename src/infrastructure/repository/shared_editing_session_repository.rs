use std::cell::RefCell;
use std::rc::Rc;

use crate::application::interface::editing_session_repository::EditingSessionRepository;
use crate::application::types::editing_session::CommonEditingSession;
use crate::domain::value_object::session_id::SessionId;
use crate::infrastructure::repository::editing_session_repository::SAM2EditingSessionRepository;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

#[derive(Clone)]
pub struct SharedEditingSessionRepository {
    sessions: Rc<RefCell<SAM2EditingSessionRepository>>,
}

impl SharedEditingSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Rc::new(RefCell::new(SAM2EditingSessionRepository::new())),
        }
    }
}

impl EditingSessionRepository for SharedEditingSessionRepository {
    type InferenceContext = SAM2InferenceContext;
    type StaticContext = SAM2StaticContext;

    fn get(
        &mut self,
        session_id: &SessionId,
    ) -> Option<CommonEditingSession<Self::StaticContext, Self::InferenceContext>> {
        self.sessions.borrow_mut().get(session_id)
    }

    fn save(
        &mut self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    ) {
        self.sessions.borrow_mut().save(session_id, editing_session);
    }
}
