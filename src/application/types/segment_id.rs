use uuid::Uuid;

use crate::application::errors::validation::segment_input_errors::SegmentInputErrors;

pub struct SegmentId {
    id: Uuid,
}

impl SegmentId {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self { id: id }
    }

    pub fn from_str(id: &str) -> Result<Self, SegmentInputErrors> {
        let id_result = Uuid::parse_str(id);
        let id = match id_result {
            Ok(uuid) => uuid,
            Err(_error) => return Err(SegmentInputErrors::InvalidSegmentId),
        };

        Ok(Self { id: id })
    }

    pub fn value(&self) -> &Uuid {
        &self.id
    }
}
