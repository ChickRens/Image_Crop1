use crate::domain::value_object::point::Point;
use crate::domain::value_object::session_id::SessionId;
use crate::domain::value_object::image_id::ImageId;

pub struct SegmentInput{
    session_id: SessionId,
    image_id: ImageId,
    point: Option<Point>
}

impl SegmentInput {
    pub fn into_parts(self) -> (SessionId, ImageId, Option<Point>) {
        (self.session_id, self.image_id, self.point)
    }
}