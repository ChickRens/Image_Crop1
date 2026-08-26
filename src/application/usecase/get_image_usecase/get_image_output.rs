use crate::domain::value_object::image_size::image_size::ImageSize;

pub struct GetImageOutput<'a> {
    data: &'a [u8],
    size: ImageSize,
}

impl<'a> GetImageOutput<'a> {
    pub fn new(data: &'a [u8], size: ImageSize) -> Self {
        Self { data: data , size}
    }

    pub fn image_data(&self) -> &'a[u8] {
        self.data
    }
}
