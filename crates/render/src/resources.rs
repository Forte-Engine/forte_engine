use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct Handle<T> { pub hash: u64, pub data: PhantomData<T> }

impl <T> Handle<T> {
    /// Creates a new handle instaces with the given hash
    /// 
    /// Arguments
    /// * hash - The hash of the this handle.
    fn new(hash: u64) -> Self { Self { hash, data: PhantomData::default() } }
}

#[derive(Debug, Clone)]
pub struct ResourceCache<T> {
    assets: HashMap<u64, T>,
}

impl<T> ResourceCache<T> {
    /// Creates a new empty resource cache.
    pub fn new() -> Self { Self { assets: HashMap::new() } }

    /// Creates a u64 hash from the given path ID string
    fn hash_path(path: String) -> u64 {
        let mut output: u64 = 0;
        path.chars().for_each(|char| {
            let digit = char.to_digit(36);
            if digit.is_some() {
                output = output + digit.unwrap() as u64;
            }
        });
        return output;
    }

    /// Returns a handle of the object with the given path ID, calling the given load function if necessary.
    /// 
    /// Arguments
    /// * path - The path ID for this object.
    /// * load - The load function that will be called if this object does not exist in the cache yet.
    pub fn load<F>(&mut self, path: impl Into<String>, load: F) -> Handle<T> where F: Fn() -> T {
        // hash the path so we can see if an insert is required
        let hash = Self::hash_path(path.into());

        // only insert the hash if necessary
        if self.assets.contains_key(&hash) {
            return Handle::new(hash);
        } else {
            // load and save
            self.assets.insert(hash, load());

            // return asset handle
            return Handle::new(hash);
        }
    }

    /// Get a resource from the cache using the given handle
    /// 
    /// Arguments
    /// * handle - The handle to be used to get the resource from the cache.
    pub fn get(&self, handle: &Handle<T>) -> Option<&T> { self.assets.get(&handle.hash) }

    /// Gets a mutable resource from the cache using the given handle.
    /// 
    /// Arguments
    /// * handle - The handle to be used to get the resource from the cache.
    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> { self.assets.get_mut(&handle.hash) }

    /// Inserts an object into the resource cache with the given hash value.
    /// 
    /// Arguments:
    /// * hash - The hash this object will be saved with.
    /// * value - The value to be inserted into the cache.
    pub fn insert(&mut self, hash: u64, value: T) { self.assets.insert(hash, value); }

    /// Replaces the resource with the given handle with the given value.
    /// 
    /// Arguments
    /// * handle - The handle of which the given value will be replaced.
    /// * value - The value that will be replacing the old value.
    pub fn replace(&mut self, handle: &Handle<T>, value: T) { self.assets.insert(handle.hash, value); }
}
