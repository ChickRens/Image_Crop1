use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::session_id::SessionId;

pub struct UploadOutput{
    session_id: SessionId,
    image_id: ImageId,
}

impl UploadOutput {
    pub fn new(session_id: SessionId, image_id: ImageId) -> Self {
        Self { session_id, image_id }
    }

    pub fn into_parts(self) -> (SessionId, ImageId) {
        (self.session_id, self.image_id)
    }
}