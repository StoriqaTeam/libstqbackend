pub mod in_memory;
pub mod redis;
pub mod typed;

use failure::Fail;
use std::marker::PhantomData;

pub trait Cache<T> {
    type Error: Fail;

    fn get(&self, key: &str) -> Result<Option<T>, Self::Error>;

    fn set(&self, key: &str, value: T) -> Result<(), Self::Error>;

    fn remove(&self, key: &str) -> Result<bool, Self::Error>;
}

pub trait CacheSingle<T> {
    type Error: Fail;

    fn get(&self) -> Result<Option<T>, Self::Error>;

    fn set(&self, value: T) -> Result<(), Self::Error>;

    fn remove(&self) -> Result<bool, Self::Error>;
}

pub struct CachedSingle<C, E, T>
where
    C: Cache<T, Error = E>,
    E: Fail,
{
    backend: C,
    phantom_e: PhantomData<E>,
    phantom_t: PhantomData<T>,
}

impl<C, E, T> From<C> for CachedSingle<C, E, T>
where
    C: Cache<T, Error = E>,
    E: Fail,
{
    fn from(cache: C) -> CachedSingle<C, E, T> {
        CachedSingle {
            backend: cache,
            phantom_e: PhantomData,
            phantom_t: PhantomData,
        }
    }
}

impl<C, E: Fail, T> CacheSingle<T> for CachedSingle<C, E, T>
where
    C: Cache<T, Error = E>,
    E: Fail,
{
    type Error = E;

    fn get(&self) -> Result<Option<T>, Self::Error> {
        self.backend.get("")
    }

    fn set(&self, value: T) -> Result<(), Self::Error> {
        self.backend.set("", value)
    }

    fn remove(&self) -> Result<bool, Self::Error> {
        self.backend.remove("")
    }
}
