use std::time::{Duration, Instant};

use dashmap::{DashMap, mapref::one::RefMut};

use crate::{
    application::{
        interface::{clock::AppClock, delete_expired_repository::DeleteExpiredRepository},
        types::entry::{Entry, EntryGuard},
    },
    domain::{
        entity::original_image::OriginalImage,
        repository::original_image_repository::{
            error::OriginalImageRepositoryError, repository::OriginalImageRepository,
        },
        value_object::image_id::image_id::ImageId,
    },
};

pub struct OriginalImageRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    images: DashMap<ImageId, Entry<OriginalImage>>,
    clock: Clock,
}

impl<Clock> OriginalImageRepository for OriginalImageRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    type Guard<'a>
        = EntryGuard<RefMut<'a, ImageId, Entry<OriginalImage>>, OriginalImage>
    where
        Self: 'a;

    fn save(&self, image: OriginalImage) {
        let key = image.image_id();
        let entry = Entry::new(image, self.clock.now());
        self.images.insert(key, entry);
    }

    fn get<'a>(
        &'a self,
        image_id: &ImageId,
    ) -> Result<Self::Guard<'a>, OriginalImageRepositoryError> {
        self.images
            .get_mut(&image_id)
            .map(|entry| EntryGuard::new(entry, self.clock.now()))
            .ok_or(OriginalImageRepositoryError::ImageNotFound)
    }
}

impl<Clock> DeleteExpiredRepository for OriginalImageRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.images.retain(|_, entry| !entry.is_expired(now, ttl));
    }
}

impl<Clock> OriginalImageRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    pub fn new(clock: Clock) -> Self {
        Self {
            images: DashMap::new(),
            clock,
        }
    }
}
