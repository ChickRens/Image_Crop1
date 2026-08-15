use crate::{
    application::{
        interface::editing_session_repository::error::EditingSessionRepositoryError,
        types::editing_session::session::CommonEditingSession,
    },
    domain::value_object::session_id::session_id::SessionId,
};

pub trait EditingSessionRepository {
    type StaticContext;
    type InferenceContext;

    fn save(
        &self,
        session_id: &SessionId,
        editing_session: CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
    );
    fn get(
        &self,
        session_id: &SessionId,
    ) -> Result<
        CommonEditingSession<Self::StaticContext, Self::InferenceContext>,
        EditingSessionRepositoryError,
    >;
}
