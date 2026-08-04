use crate::domain::value_object::image_size::error::ImageSizeError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ImageSize {
    height: u16,
    width: u16,
}

impl ImageSize {
    const MIN_DIMENSION: u16 = 2;
    const MAX_DIMENSION: u16 = 3000;

    pub fn new(height: u16, width: u16) -> Result<Self, ImageSizeError> {
        match (height, width) {
            (h, _) if h < Self::MIN_DIMENSION => Err(ImageSizeError::ShortHeight),
            (h, _) if h > Self::MAX_DIMENSION => Err(ImageSizeError::LongHeight),
            (_, w) if w < Self::MIN_DIMENSION => Err(ImageSizeError::ShortWidth),
            (_, w) if w > Self::MAX_DIMENSION => Err(ImageSizeError::LongWidth),
            _ => Ok(Self { height, width }),
        }
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn fit_within(&self, max_height: u16, max_width: u16) -> Self {
        let scale: f32 =
            (max_height as f32 / self.height as f32).min(max_width as f32 / self.width as f32);

        Self {
            height: (self.height as f32 * scale) as u16,
            width: (self.width as f32 * scale) as u16,
        }
    }
}
