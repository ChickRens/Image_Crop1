use uuid::Uuid;

use crate::domain::value_object::image_id::error::ImageIdError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ImageId {
    id: Uuid,
}

impl ImageId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self { id: id }
    }

    pub fn from_str(id: &str) -> Result<Self, ImageIdError> {
        let id_result = Uuid::parse_str(id);
        let id = match id_result {
            Ok(uuid) => uuid,
            Err(_error) => return Err(ImageIdError::InvalidImageId),
        };

        Ok(Self { id: id })
    }

    pub fn value(&self) -> &Uuid {
        &self.id
    }
}
