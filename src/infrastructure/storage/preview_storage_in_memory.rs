use std::{ops::{Deref, DerefMut}, sync::Arc, time::{Duration, Instant}};

use dashmap::{DashMap, mapref::one::RefMut};

use crate::{
    application::{
        interface::{clock::AppClock, delete_expired_repository::DeleteExpiredRepository, preview_storage::{error::PreviewStorageError, storage::PreviewStorage}}, types::{entry::{Entry, EntryGuard}, preview_image::PreviewImage},
    }, domain::value_object::image_id::image_id::ImageId,
};

#[derive(Debug)]
pub struct PreviewStorageInMemory<Clock>
where 
    Clock: AppClock,
{
    original_storage: PreviewOriginalStorage<Clock>,
    segmented_storage: PreviewSegmentedStorage<Clock>,
}

impl<Clock> PreviewStorage for PreviewStorageInMemory<Clock>
where 
    Clock: AppClock,
{
    type Guard<'a> = PreviewGuard<'a>
    where
        Self: 'a;

    fn save_as_original(&self, image: PreviewImage) {
        self.original_storage.save(image);
    }
    
    fn save_as_segmented(&self, image: PreviewImage) {
        self.segmented_storage.save(image);
    }

    fn get<'a>(&'a self, image_id: ImageId) -> Result<Self::Guard<'a>, PreviewStorageError> {
        let original = self.original_storage.get(image_id);
        let segmented = self.segmented_storage.get(image_id);
        match (original, segmented) {
            (None ,None) => Err(PreviewStorageError::ImageNotFound),
            (Some(guard), None) => Ok(guard),
            (None, Some(guard)) => Ok(guard),
            (Some(_), Some(guard_segmented)) => Ok(guard_segmented),
        }
    }
}

impl<Clock> PreviewStorageInMemory<Clock>
where 
    Clock: AppClock + Clone,
{
    pub fn new(clock: Clock) -> Self {
        Self { original_storage: 
            PreviewOriginalStorage { image: DashMap::new(), clock: clock.clone() },
               segmented_storage:
            PreviewSegmentedStorage { image: DashMap::new(), clock: clock.clone() }
            }
    }
}

impl<Clock> DeleteExpiredRepository for PreviewStorageInMemory<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.original_storage.delete_expired(now, ttl);
    }
}

pub enum PreviewGuard<'a> {
    Original(EntryGuard<RefMut<'a, ImageId, Entry<PreviewImage>>, PreviewImage>),
    Segmented(EntryGuard<OwnedGuard<Entry<PreviewImage>>, PreviewImage>)
}

impl<'a> Deref for PreviewGuard<'a> {
    type Target = PreviewImage;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Original(guard) => guard.deref(),
            Self::Segmented(guard) => guard.deref()
        }
    }
}

impl<'a> DerefMut for PreviewGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Original(guard) => guard.deref_mut(),
            Self::Segmented(guard) => guard.deref_mut(),
        }
    }
}

#[derive(Debug)]
struct PreviewOriginalStorage<Clock>
where 
    Clock: AppClock
{
    image: DashMap<ImageId, Entry<PreviewImage>>,
    clock: Clock,
}

impl<Clock> PreviewOriginalStorage<Clock>
where
    Clock: AppClock
{
    fn save(&self, original: PreviewImage) {
        let image_id = original.image_id();
        let entry = Entry::new(original, self.clock.now());
        self.image.insert(image_id, entry);
    }

    fn get(&self, image_id: ImageId) -> Option<PreviewGuard<'_>> {
        self.image
            .get_mut(&image_id)
            .map(|entry| {
                EntryGuard::new(entry, self.clock.now())
            })
            .map(|guard|{
                PreviewGuard::Original(guard)
            })
    }

    fn new(clock: Clock) -> Self {
        Self { image: DashMap::new(), clock }
    }
}

impl<Clock> DeleteExpiredRepository for PreviewOriginalStorage<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.image.retain(|_, entry|{
            !entry.is_expired(now, ttl)
        });
    }
}

#[derive(Debug)]
struct PreviewSegmentedStorage<Clock>
where
    Clock: AppClock,
{
    image: DashMap<ImageId, Entry<PreviewImage>>,
    clock: Clock,
}

impl<Clock> PreviewSegmentedStorage<Clock>
where
    Clock: AppClock,
{
    pub fn save(&self, segmented: PreviewImage) {
        let image_id = segmented.image_id();
        let entry = Entry::new(segmented, self.clock.now());
        self.image.insert(image_id, entry);
    }

    fn get<'a>(&'a self, image_id: ImageId) -> Option<PreviewGuard<'a>> {
        self.image
            .remove(&image_id)
            .map(|(_, entry)|{
                EntryGuard::new(OwnedGuard {value: entry}, self.clock.now())
            })
            .map(|guard|{
                PreviewGuard::Segmented(guard)
            })
    }
}

impl<Clock> DeleteExpiredRepository for PreviewSegmentedStorage<Clock>
where
    Clock: AppClock,
{
    fn delete_expired(&self, now: Instant, ttl: Duration) {
        self.image.retain(|_, entry|{
            !entry.is_expired(now, ttl)
        });
    }
}

pub struct OwnedGuard<T>{
    value: T
}

impl<T> Deref for OwnedGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for OwnedGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Debug, Clone)]
pub struct SharedPreviewStorage<Clock>
where
    Clock: AppClock,
{
    storage: Arc<PreviewStorageInMemory<Clock>>,
}

impl<Clock> PreviewStorage for SharedPreviewStorage<Clock>
where
    Clock: AppClock,
{
    type Guard<'a> = PreviewGuard<'a>
        where
            Self: 'a;

    fn save_as_original(&self, original: PreviewImage) {
        self.storage.save_as_original(original);
    }

    fn save_as_segmented(&self, segmented: PreviewImage) {
        self.storage.save_as_segmented(segmented);
    }

    fn get<'a>(&'a self, image_id: ImageId) -> Result<Self::Guard<'a>, PreviewStorageError> {
        self.storage.get(image_id)
    }
}

impl<Clock> SharedPreviewStorage<Clock>
where
    Clock: AppClock + Clone,
{
    pub fn new(clock: Clock) -> Self {
        Self {
            storage: Arc::new(PreviewStorageInMemory::new(clock)),
        }
    }
}
