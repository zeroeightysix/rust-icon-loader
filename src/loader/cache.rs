use std::collections::HashMap;

use super::Mutable;

pub(super) trait ImplCache<T> {
    fn get(&self, key: &str) -> Option<T>;
    fn insert(&self, key: String, value: T);
    fn clear(&self);
    fn contains_key(&self, key: &str) -> bool;
}

#[cfg(not(feature = "sync"))]
impl<T> ImplCache<T> for Mutable<HashMap<String, T>>
where
    T: Clone,
{
    fn get(&self, key: &str) -> Option<T> {
        self.borrow().get(key).map(T::to_owned)
    }

    fn insert(&self, key: String, value: T) {
        self.borrow_mut().insert(key, value);
    }

    fn clear(&self) {
        self.borrow_mut().clear()
    }

    fn contains_key(&self, key: &str) -> bool {
        self.borrow().contains_key(key)
    }
}

#[cfg(feature = "sync")]
impl<T> ImplCache<T> for Mutable<HashMap<String, T>>
where
    T: Clone,
{
    fn get(&self, key: &str) -> Option<T> {
        self.read().unwrap().get(key).map(T::to_owned)
    }

    fn insert(&self, key: String, value: T) {
        self.write().unwrap().insert(key, value);
    }

    fn clear(&self) {
        self.write().unwrap().clear()
    }

    fn contains_key(&self, key: &str) -> bool {
        self.read().unwrap().contains_key(key)
    }
}
