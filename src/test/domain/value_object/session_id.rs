#[cfg(test)]
mod image_size_tests {
    use uuid::Uuid;

    use crate::domain::value_object::session_id::{error::SessionIdError, session_id::SessionId};

    #[test]
    fn test_normal_convert_from_uuid() {
        let uuid: Uuid = Uuid::new_v4();
        let id: SessionId = SessionId::from_uuid(uuid);

        assert_eq!(id.value(), &uuid)
    }

    #[test]
    fn test_normal_convert_from_str() {
        let uuid = Uuid::parse_str("1754ca13-617a-46be-bad2-a83eaedccb77");
        let id = SessionId::from_str("1754ca13-617a-46be-bad2-a83eaedccb77");

        assert_eq!(id.unwrap().value(), &uuid.unwrap())
    }

    #[test]
    fn test_invalid_str() {
        let id = SessionId::from_str("asga129470tgiaehdsg9g8sph");

        assert_eq!(id, Err(SessionIdError::InvalidSessionId))
    }
}
