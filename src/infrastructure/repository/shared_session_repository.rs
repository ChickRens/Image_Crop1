use std::sync::Arc;
use std::time::{Duration, Instant};

use dashmap::mapref::one::RefMut;

use crate::application::interface::{
    clock::AppClock,
    delete_expired_repository::DeleteExpiredRepository,
};
use crate::application::types::entry::{Entry, EntryGuard};
use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::repository::session_repository::repository::SessionRepository;
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

#[derive(Clone)]
pub struct SharedSessionRepository<Clock>
where
    Clock: AppClock,
{
    sessions: Arc<SessionRepositoryInMemory<Clock>>,
}

impl<Clock> SessionRepository for SharedSessionRepository<Clock>
where
    Clock: AppClock,
{
    type Guard<'a> = EntryGuard<RefMut<'a, SessionId, Entry<Session>>, Session>
    where
        Self: 'a;

    fn save(&self, session: Session) {
        self.sessions.save(session);
    }

    fn get<'a>(
        &'a self,
        session_id: &SessionId,
    ) -> Result<Self::Guard<'a>, SessionRepositoryError> {
        self.sessions.get(session_id)
    }
}

impl<Clock> DeleteExpiredRepository for SharedSessionRepository<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.sessions.delete_expired(now, ttl);
    }
}

impl<Clock> SharedSessionRepository<Clock>
where
    Clock: AppClock,
{
    pub fn new(clock: Clock) -> Self {
        Self {
            sessions: Arc::new(SessionRepositoryInMemory::new(clock)),
        }
    }
}
