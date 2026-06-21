use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::session_id::SessionId;

pub struct UndoOutput {
    session_id: SessionId,
    image_id: ImageId,
}

impl UndoOutput {
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
