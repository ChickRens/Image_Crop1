#[cfg(test)]
mod editing_session_repository_in_memory_test {
    use ndarray::Array4;

    use crate::{
        application::{
            interface::editing_session_repository::{
                error::EditingSessionRepositoryError, repository::EditingSessionRepository,
            },
            types::{
                editing_session::session::CommonEditingSession,
                inference_context_history::InferenceContextHistory, point_history::PointHistory,
            },
        },
        domain::value_object::session_id::session_id::SessionId,
        infrastructure::{
            repository::editing_session_repository::SAM2EditingSessionRepository,
            segmenter::sam2_data::{
                HighResFeatureS0, HighResFeatureS1, ImageEmbeddings, SAM2InferenceContext,
                SAM2StaticContext,
            },
        },
    };

    fn create_editing_session() -> CommonEditingSession<SAM2StaticContext, SAM2InferenceContext> {
        let static_context = SAM2StaticContext::new(
            ImageEmbeddings::new(Array4::zeros((1, 1, 1, 1))),
            HighResFeatureS0::new(Array4::zeros((1, 1, 1, 1))),
            HighResFeatureS1::new(Array4::zeros((1, 1, 1, 1))),
        );

        let inference_context = SAM2InferenceContext::new(None);
        let inference_history = InferenceContextHistory::new(5);

        CommonEditingSession::new(
            PointHistory::new(5),
            static_context,
            inference_history,
            inference_context,
        )
    }

    #[test]
    fn test_normal_get() {
        let repo = SAM2EditingSessionRepository::new();
        let session_id = SessionId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let editing_session = create_editing_session();

        repo.save(&session_id, editing_session.clone());

        let got = repo.get(&session_id).unwrap();

        assert_eq!(got, editing_session);
    }

    #[test]
    fn test_unknown_session_get() {
        let repo = SAM2EditingSessionRepository::new();
        let session_id = SessionId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let unknown_session_id =
            SessionId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();
        let editing_session = create_editing_session();

        repo.save(&session_id, editing_session);

        let got = repo.get(&unknown_session_id);

        assert_eq!(
            got,
            Err(EditingSessionRepositoryError::EditingSessionNotFound)
        );
    }

    #[test]
    fn test_overwrite_save_existing_session() {
        let repo = SAM2EditingSessionRepository::new();
        let session_id = SessionId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let old_session = create_editing_session();
        let new_session = create_editing_session();

        repo.save(&session_id, old_session.clone());
        repo.save(&session_id, new_session.clone());

        let got = repo.get(&session_id).unwrap();

        assert_eq!(got, new_session);
    }
}
