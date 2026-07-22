use std::{cell::RefCell, rc::Rc};

use crate::domain::entity::session::Session;
use crate::domain::value_object::session_id::SessionId;

pub trait SessionRepository {
    fn save(&self, session: Session);
    fn get(&self, session_id: &SessionId) -> Option<Session>;
}

impl<T> SessionRepository for Rc<RefCell<T>>
where
    T: SessionRepository,
{
    fn save(&self, session: Session) {
        self.borrow_mut().save(session);
    }

    fn get(&self, session_id: &SessionId) -> Option<Session> {
        self.borrow().get(session_id)
    }
}
