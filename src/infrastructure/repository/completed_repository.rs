use std::{
    sync::Arc, time::{Duration, Instant},
};

use dashmap::{DashMap, mapref::one::RefMut};

use crate::{
    application::{
        interface::{clock::AppClock, completed_image_repository::{
            error::CompletedImageRepositoryError, repository::CompletedImageRepository,
        }, delete_expired_repository::DeleteExpiredRepository}, types::{completed_image::CompletedImage, entry::{Entry, EntryGuard}},
    }, domain::value_object::image_id::image_id::ImageId,
};

#[derive(Debug)]
pub struct CompletedRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    image: DashMap<ImageId, Entry<CompletedImage>>,
    clock: Clock,
}

impl<Clock> CompletedImageRepository for CompletedRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    type Guard<'a> = EntryGuard<RefMut<'a, ImageId, Entry<CompletedImage>>, CompletedImage>
        where
            Self: 'a;

    fn save(&self, completed_image: CompletedImage) {
        let image_id = completed_image.image_id();

        let entry = Entry::new(completed_image, self.clock.now());
        self.image.insert(image_id, entry);
    }

    fn get<'a>(&'a self, image_id: ImageId) -> Result<Self::Guard<'a>, CompletedImageRepositoryError> {
        self.image
            .get_mut(&image_id)
            .map(|entry|{
                EntryGuard::new(entry, self.clock.now())
            })
            .ok_or(CompletedImageRepositoryError::ImageNotFound)
    }
}

impl<Clock> DeleteExpiredRepository for CompletedRepositoryInMemory<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.image.retain(|_ ,image| {
            !image.is_expired(now, ttl)
        });
    }
}

impl<Clock> CompletedRepositoryInMemory<Clock>
where
    Clock: AppClock
{
    pub fn new(clock: Clock) -> Self {
        Self {
            image: DashMap::new(),
            clock
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedCompletedImageRepository<Clock>
where
    Clock: AppClock,
{
    repository: Arc<CompletedRepositoryInMemory<Clock>>,
}

impl<Clock> CompletedImageRepository for SharedCompletedImageRepository<Clock>
where
    Clock: AppClock,
{
    type Guard<'a> = EntryGuard<RefMut<'a, ImageId, Entry<CompletedImage>>, CompletedImage>
        where
            Self: 'a;

    fn get<'a>(&'a self, image_id: ImageId) -> Result<Self::Guard<'a>, CompletedImageRepositoryError> {
        self.repository.get(image_id)
    }

    fn save(&self, completed_image: CompletedImage) {
        self.repository.save(completed_image);
    }
}

impl<Clock> SharedCompletedImageRepository<Clock>
where
    Clock: AppClock,
{
    pub fn new(clock: Clock) -> Self {
        Self {
            repository: Arc::new(CompletedRepositoryInMemory::new(clock))
        }
    }
}
