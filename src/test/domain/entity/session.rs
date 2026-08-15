#[cfg(test)]
mod session_tests {
    use uuid::Uuid;

    use crate::domain::{
        entity::session::session::Session,
        value_object::{image_id::image_id::ImageId, session_id::session_id::SessionId},
    };

    #[test]
    fn test_session_new_and_accessors() {
        let session_id = SessionId::from_uuid(Uuid::new_v4());
        let image_id = ImageId::from_uuid(Uuid::new_v4());
        let session = Session::new(session_id.clone(), image_id.clone());

        assert_eq!(session.session_id(), &session_id);
        assert_eq!(session.image_id(), &image_id);
    }
}
