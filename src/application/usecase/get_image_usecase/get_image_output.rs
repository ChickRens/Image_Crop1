use crate::domain::value_object::image_size::image_size::ImageSize;

pub struct GetImageOutput {
    data: Vec<u8>,
    size: ImageSize,
}

impl GetImageOutput {
    pub fn new(data: Vec<u8>, size: ImageSize) -> Self {
        Self { data: data , size}
    }

    pub fn image_data(self) -> (Vec<u8>, ImageSize) {
        (self.data, self.size)
    }
}
