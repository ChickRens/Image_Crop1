use std::ops::DerefMut;

use crate::domain::entity::original_image::OriginalImage;
use crate::domain::repository::original_image_repository::error::OriginalImageRepositoryError;
use crate::domain::value_object::image_id::image_id::ImageId;

pub trait OriginalImageRepository {
    type Guard<'a>: DerefMut<Target = OriginalImage>
    where
        Self: 'a;

    fn save(&self, image: OriginalImage);
    fn get<'a>(
        &'a self,
        image_id: &ImageId,
    ) -> Result<Self::Guard<'a>, OriginalImageRepositoryError>;
}
