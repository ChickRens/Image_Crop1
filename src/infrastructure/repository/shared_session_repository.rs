use std::cell::RefCell;
use std::rc::Rc;

use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::SessionRepository;
use crate::domain::value_object::session_id::SessionId;
use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

#[derive(Clone)]
pub struct SharedSessionRepository {
    sessions: Rc<RefCell<SessionRepositoryInMemory>>,
}

impl SessionRepository for SharedSessionRepository {
    fn save(&mut self, session: Session) {
        self.sessions.borrow_mut().save(session);
    }
    fn get(&self, session_id: &SessionId) -> Option<Session> {
        self.sessions.borrow().get(session_id)
    }
}

impl SharedSessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: Rc::new(RefCell::new(SessionRepositoryInMemory::new())),
        }
    }
}
