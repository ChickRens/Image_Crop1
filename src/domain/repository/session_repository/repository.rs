use std::ops::DerefMut;

use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::value_object::session_id::session_id::SessionId;

pub trait SessionRepository {
    type Guard<'a>: DerefMut<Target = Session>
    where
        Self: 'a;

    fn save(&self, session: Session);
    fn get<'a>(&'a self, session_id: &SessionId)
    -> Result<Self::Guard<'a>, SessionRepositoryError>;
}
