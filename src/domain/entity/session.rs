use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::session_id::SessionId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    session_id: SessionId,
    image_id: ImageId,
}

impl Session {
    pub fn new(session_id: SessionId, image_id: ImageId) -> Self {
        Self {
            session_id,
            image_id,
        }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn has_image_id(&self, image_id: &ImageId) -> bool{
        image_id==&self.image_id
    }

    pub fn image_id(&self) -> &ImageId {
        &self.image_id
    }
}
