use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::point::Point;
use crate::domain::value_object::session_id::SessionId;

pub struct SegmentInput {
    session_id: SessionId,
    image_id: ImageId,
    points: Vec<Point>,
}

impl SegmentInput {
    pub fn into_parts(self) -> (SessionId, ImageId, Vec<Point>) {
        (self.session_id, self.image_id, self.points)
    }
}
