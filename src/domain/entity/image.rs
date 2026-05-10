use crate::domain::value_object::image_data::ImageData;
use crate::domain::value_object::image_id::ImageId;
use crate::domain::value_object::image_size::ImageSize;
use crate::domain::value_object::point::Point;
use crate::domain::value_object::point_history::PointHistory;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image {
    image_data: ImageData,
    image_id: ImageId,
    image_size: ImageSize,
    point_history: PointHistory,
}

impl Image {
    pub fn new(image_data: ImageData, id: ImageId, size: ImageSize, max_history: usize) -> Self {
        Self {
            image_data: image_data,
            image_id: id,
            image_size: size,
            point_history: PointHistory::new(max_history),
        }
    }

    pub fn image_data(&self) -> &ImageData {
        &self.image_data
    }

    pub fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    pub fn image_size(&self) -> &ImageSize {
        &self.image_size
    }

    pub fn into_data(self) -> (ImageData, ImageId, ImageSize) {
        (self.image_data, self.image_id, self.image_size)
    }

    pub fn undo(&mut self) -> Option<&Point> {
        self.point_history.undo()
    }

    pub fn redo(&mut self) -> Option<&Point> {
        self.point_history.redo()
    }
}
