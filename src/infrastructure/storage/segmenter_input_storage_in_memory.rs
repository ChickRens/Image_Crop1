use std::time::{Duration, Instant};

use dashmap::{DashMap, mapref::one::RefMut};

use crate::{
    application::{
        interface::{
            clock::AppClock,
            delete_expired_repository::DeleteExpiredRepository,
            segmenter_input_image_storage::{
                error::SegmenterInputImageStorageError, storage::SegmenterInputImageStorage,
            },
        },
        types::{
            entry::{Entry, EntryGuard},
            segmenter_input_image::SegmenterInputImage,
        },
    },
    domain::value_object::image_id::image_id::ImageId,
};

#[derive(Debug)]
pub struct SegmenterInputStorageInMemory<Clock>
where
    Clock: AppClock,
{
    images: DashMap<ImageId, Entry<SegmenterInputImage>>,
    clock: Clock,
}

impl<Clock> SegmenterInputImageStorage for SegmenterInputStorageInMemory<Clock>
where
    Clock: AppClock,
{
    type Guard<'a>
        = EntryGuard<RefMut<'a, ImageId, Entry<SegmenterInputImage>>, SegmenterInputImage>
    where
        Self: 'a;

    fn save(&self, image: SegmenterInputImage) {
        let id = image.image_id().clone();
        let entry = Entry::new(image, self.clock.now());
        self.images.insert(id, entry);
    }

    fn get<'a>(
        &'a self,
        image_id: ImageId,
    ) -> Result<Self::Guard<'a>, SegmenterInputImageStorageError> {
        self.images
            .get_mut(&image_id)
            .map(|entry| EntryGuard::new(entry, self.clock.now()))
            .ok_or(SegmenterInputImageStorageError::ImageNotFound)
    }
}

impl<Clock> DeleteExpiredRepository for SegmenterInputStorageInMemory<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.images.retain(|_, entry| !entry.is_expired(now, ttl));
    }
}

impl<Clock> SegmenterInputStorageInMemory<Clock>
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
