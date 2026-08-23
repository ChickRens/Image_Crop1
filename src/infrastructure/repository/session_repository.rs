use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::repository::session_repository::repository::SessionRepository;
use crate::domain::value_object::session_id::session_id::SessionId;

pub struct SessionRepositoryInMemory {
    sessions: Mutex<HashMap<SessionId, Session>>,
}

impl SessionRepository for SessionRepositoryInMemory {
    fn save(&self, session: Session) {
        let mut sessions = self
            .sessions
            .lock()
            .expect("SessionRepositoryInMemory is Poisoned");
        sessions.insert(session.session_id().clone(), session);
    }

    fn get(&self, session_id: &SessionId) -> Result<Session, SessionRepositoryError> {
        let sessions = self
            .sessions
            .lock()
            .expect("SessionRepositoryInMemory is Poisoned");
        sessions
            .get(session_id)
            .cloned()
            .ok_or(SessionRepositoryError::SessionNotFound)
    }
}

impl SessionRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}
