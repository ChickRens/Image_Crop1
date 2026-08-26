use crate::domain::value_object::{image_id::image_id::ImageId, session_id::session_id::SessionId};

pub struct PrepareSegmentInput {
    image_id: ImageId,
    session_id: SessionId
}

impl PrepareSegmentInput {
    pub fn new(image_id: ImageId, session_id: SessionId) -> Self {
        Self { image_id, session_id }
    }

    pub fn image_id(&self) -> ImageId {
        self.image_id
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}