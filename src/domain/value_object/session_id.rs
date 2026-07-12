use uuid::Uuid;

use crate::domain::errors::session_errors::SessionErrors;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SessionId {
    id: Uuid,
}

impl SessionId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self { id: id }
    }

    pub fn from_str(id: &str) -> Result<Self, SessionErrors> {
        let id_result = Uuid::parse_str(id);
        let id = match id_result {
            Ok(uuid) => uuid,
            Err(_error) => return Err(SessionErrors::InvalidSessionId),
        };

        Ok(Self { id: id })
    }

    pub fn value(&self) -> &Uuid {
        &self.id
    }
}
