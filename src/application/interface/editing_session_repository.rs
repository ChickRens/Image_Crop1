use crate::application::types::editing_session::CommonEditingSession;
use crate::domain::value_object::session_id::SessionId;

pub trait EditingSessionRepository {
    type StaticContext;
    type InferenceContext;

    fn save(
        &mut self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    );
    fn get(
        &self,
        session_id: &SessionId,
    ) -> Option<&CommonEditingSession<Self::StaticContext, Self::InferenceContext>>;
    fn get_mut(
        &mut self,
        session_id: &SessionId,
    ) -> Option<&mut CommonEditingSession<Self::StaticContext, Self::InferenceContext>>;
}
