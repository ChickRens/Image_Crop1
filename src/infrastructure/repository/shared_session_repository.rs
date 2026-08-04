use std::sync::Arc;

use crate::domain::entity::session::session::Session;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::repository::session_repository::repository::SessionRepository;
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

#[derive(Clone)]
pub struct SharedSessionRepository {
    sessions: Arc<SessionRepositoryInMemory>,
}

impl SessionRepository for SharedSessionRepository {
    fn save(&self, session: Session) {
        self.sessions.save(session);
    }
    fn get(&self, session_id: &SessionId) -> Result<Session, SessionRepositoryError> {
        self.sessions.get(session_id)
    }
}

impl SharedSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(SessionRepositoryInMemory::new()),
        }
    }
}
