use std::{ops::{Deref, DerefMut}, time::{Duration, Instant}};

#[derive(Debug)]
pub struct Entry<T> {
    value: T,
    last_accessed: Instant
}

impl<T> Entry<T> {
    pub fn touch(&mut self) {
        self.last_accessed = Instant::now()
    }

    pub fn is_expired(&self, now: Instant, ttl: Duration) -> bool {
        (now - self.last_accessed) > ttl
    }

    pub fn new(value: T) -> Self {
        Self { value, last_accessed: Instant::now() }
    }
}

impl<T> Deref for Entry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Entry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
