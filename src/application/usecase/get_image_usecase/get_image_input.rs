use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_kind::ImageKind;
use crate::domain::value_object::session_id::SessionId;

pub struct GetImageInput{
    session_id: SessionId,
    image_id: ImageId,
    image_kind: ImageKind
}

impl GetImageInput {
    pub fn new(session_id: SessionId, image_id: ImageId, image_kind: ImageKind) -> Self {
        Self {
            session_id,
            image_id,
            image_kind,
        }
    }

    pub fn into_session_id_image_id_image_kind(self) -> (SessionId,ImageId,ImageKind) {
        (self.session_id, self.image_id, self.image_kind)
    }
}