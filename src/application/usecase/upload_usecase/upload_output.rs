use crate::domain::value_object::{image_id::image_id::ImageId, session_id::session_id::SessionId};

#[derive(Debug, PartialEq, Eq)]
pub struct UploadOutput {
    session_id: SessionId,
    image_id: ImageId,
}

impl UploadOutput {
    pub fn new(session_id: SessionId, image_id: ImageId) -> Self {
        Self {
            session_id,
            image_id,
        }
    }

    pub fn into_session_id_and_image_id(self) -> (SessionId, ImageId) {
        (self.session_id, self.image_id)
    }
}
