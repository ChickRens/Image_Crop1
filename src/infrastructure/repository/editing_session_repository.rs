use std::time::{Duration, Instant};

use dashmap::DashMap;
use dashmap::mapref::one::RefMut;

use crate::application::interface::clock::AppClock;
use crate::application::interface::delete_expired_repository::DeleteExpiredRepository;
use crate::application::interface::editing_session_repository::error::EditingSessionRepositoryError;
use crate::application::interface::editing_session_repository::repository::EditingSessionRepository;
use crate::application::types::editing_session::session::CommonEditingSession;
use crate::application::types::entry::{Entry, EntryGuard};
use crate::domain::value_object::session_id::session_id::SessionId;
use crate::infrastructure::segmenter::sam2_data::{SAM2InferenceContext, SAM2StaticContext};

pub struct SAM2EditingSessionRepository<Clock>
where
    Clock: AppClock
{
    sessions: DashMap<SessionId, Entry<CommonEditingSession<SAM2StaticContext, SAM2InferenceContext>>>,
    clock: Clock,
}

impl<Clock> EditingSessionRepository for SAM2EditingSessionRepository<Clock>
where 
    Clock: AppClock,
{
    type StaticContext = SAM2StaticContext;
    type InferenceContext = SAM2InferenceContext;
    type Guard<'a>
        = EntryGuard<RefMut<'a, SessionId, Entry<CommonEditingSession<Self::StaticContext, Self::InferenceContext>>>, CommonEditingSession<Self::StaticContext, Self::InferenceContext>>
    where
        Self: 'a;

    fn save(
        &self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    ) {
        let entry = Entry::new(editing_session, self.clock.now());
        self.sessions.insert(*session_id, entry);
    }

    fn get<'a>(
        &'a self,
        session_id: &SessionId,
    ) -> Result<Self::Guard<'a>, EditingSessionRepositoryError> {
        self.sessions
            .get_mut(session_id)
            .map(|e| {
                EntryGuard::new(e, self.clock.now())
            })
            .ok_or(EditingSessionRepositoryError::EditingSessionNotFound)
            
    }
}

impl<C> DeleteExpiredRepository for SAM2EditingSessionRepository<C>
where
    C: AppClock
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.sessions
            .retain(|_ ,entry|{
                !entry.is_expired(now, ttl)
            });
    }
}

impl<Clock> SAM2EditingSessionRepository<Clock>
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
