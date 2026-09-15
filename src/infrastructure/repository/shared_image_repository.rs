use std::sync::Arc;

use dashmap::mapref::one::RefMut;

use crate::{
    application::{interface::clock::AppClock, types::entry::{Entry, EntryGuard}}, domain::{
        entity::original_image::OriginalImage,
        repository::original_image_repository::{
            error::OriginalImageRepositoryError, repository::OriginalImageRepository,
        },
        value_object::image_id::image_id::ImageId,
    }, infrastructure::repository::image_repository::OriginalImageRepositoryInMemory,
};

#[derive(Clone)]
pub struct SharedOriginalImageRepository<Clock>
where
    Clock: AppClock,
{
    images: Arc<OriginalImageRepositoryInMemory<Clock>>,
}

impl<Clock> SharedOriginalImageRepository<Clock>
where
    Clock: AppClock,
{
    pub fn new(clock: Clock) -> Self {
        Self {
            images: Arc::new(OriginalImageRepositoryInMemory::new(clock)),
        }
    }
}

impl<Clock> OriginalImageRepository for SharedOriginalImageRepository<Clock>
where
    Clock: AppClock,
{
    type Guard<'a> = EntryGuard<RefMut<'a, ImageId, Entry<OriginalImage>>, OriginalImage>
        where 
            Self: 'a;

    fn get<'a>(&'a self, image_id: &ImageId) -> Result<Self::Guard<'a>, OriginalImageRepositoryError> {
        self.images.get(image_id)
    }

    fn save(&self, image: OriginalImage) {
        self.images.save(image);
    }
}
