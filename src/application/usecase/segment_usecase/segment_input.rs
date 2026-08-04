use crate::domain::value_object::{image_id::image_id::ImageId, point::Point, session_id::session_id::SessionId};

pub struct SegmentInput {
    session_id: SessionId,
    image_id: ImageId,
    point: Point,
}

impl SegmentInput {
    pub fn into_parts(self) -> (SessionId, ImageId, Point) {
        (self.session_id, self.image_id, self.point)
    }

    pub fn new(session_id: SessionId, image_id: ImageId, point: Point) -> Self {
        Self {
            session_id,
            image_id,
            point,
        }
    }
}
