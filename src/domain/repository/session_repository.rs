use crate::domain::entity::session::Session;
use crate::domain::value_object::session_id::SessionId;

pub trait SessionRepository {
    fn save(&mut self, session: Session);
    fn get(&self, session_id: &SessionId) -> Option<Session>;
}
