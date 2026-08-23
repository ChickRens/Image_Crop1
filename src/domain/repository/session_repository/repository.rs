use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::value_object::session_id::session_id::SessionId;

pub trait SessionRepository {
    fn save(&self, session: Session);
    fn get(&self, session_id: &SessionId) -> Result<Session, SessionRepositoryError>;
}
