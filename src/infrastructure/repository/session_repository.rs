use std::time::{Duration, Instant};

use dashmap::{DashMap, mapref::one::RefMut};

use crate::application::{
    interface::{clock::AppClock, delete_expired_repository::DeleteExpiredRepository}, types::entry::{Entry, EntryGuard},
};
use crate::domain::entity::session::Session;
use crate::domain::repository::session_repository::error::SessionRepositoryError;
use crate::domain::repository::session_repository::repository::SessionRepository;
use crate::domain::value_object::session_id::session_id::SessionId;

pub struct SessionRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    sessions: DashMap<SessionId, Entry<Session>>,
    clock: Clock,
}

impl<Clock> SessionRepository for SessionRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    type Guard<'a> = EntryGuard<RefMut<'a, SessionId, Entry<Session>>, Session>
    where
        Self: 'a;

    fn save(&self, session: Session) {
        let session_id = session.session_id().clone();
        let entry = Entry::new(session, self.clock.now());
        self.sessions.insert(session_id, entry);
    }

    fn get<'a>(&'a self, session_id: &SessionId) -> Result<Self::Guard<'a>, SessionRepositoryError> {
        self.sessions
            .get_mut(session_id)
            .map(|entry|{
                EntryGuard::new(entry, self.clock.now())
            })
            .ok_or(SessionRepositoryError::SessionNotFound)
    }
}

impl<Clock> DeleteExpiredRepository for SessionRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.sessions.retain(|_, entry| !entry.is_expired(now, ttl));
    }
}

impl<Clock> SessionRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    pub fn new(clock: Clock) -> Self {
        Self {
            sessions: DashMap::new(),
            clock,
        }
    }
}
