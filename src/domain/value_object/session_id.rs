use uuid::Uuid;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SessionId {
    id: Uuid,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SessionErrorType {
    InvalidUuid,
}

impl SessionId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self { id: id }
    }

    pub fn from_str(id: &str) -> Result<Self, SessionErrorType> {
        let id_result = Uuid::parse_str(id);
        let id = match id_result {
            Ok(uuid) => uuid,
            Err(_error) => return Err(SessionErrorType::InvalidUuid),
        };

        Ok(Self { id: id })
    }

    pub fn value(&self) -> &Uuid {
        &self.id
    }
}
