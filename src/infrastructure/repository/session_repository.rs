use std::collections::HashMap;

use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::session_id::SessionId;

pub struct SessionRepositoryInMemory {
    sessions: HashMap<SessionId, Session>,
}

impl SessionRepository for SessionRepositoryInMemory {
    fn save(&mut self, session: Session) {
        self.sessions.insert(session.session_id().clone(), session);
    }

    fn get(&self, session_id: &SessionId) -> Option<Session> {
        self.sessions.get(session_id).cloned()
    }
}

impl SessionRepositoryInMemory{
    pub fn new() -> Self{
        Self { sessions: HashMap::new() }
    }
}