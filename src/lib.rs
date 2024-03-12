//! Simple, Lighweight and "not" thread safe Singleton instance but it's depend on the usage,
//! feel free to make thread safe wrapper
//!
//! Currently it can :
//!  * Set value to the instance with type.
//!  * Get reference value to the instance with type.
//!  * Get mutable reference value to the instance with type.
//!
//! ### Examples
//! ```
//! fn main() {
//!    // Create the Singleton instance
//!    let mut instance = singly::Singleton::new();
//!
//!    /// Set the i32 type to 12
//!    instance.set(12i32);
//!
//!    /// Get mutable reference i32 type and set it to 14
//!    let a = instance.get_mut::<i32>();
//!    *a = 14;
//!
//!    assert_eq!(instance.get::<i32>(), &14);
//!
//! }
//! ```
//!
//! ## Some tips for Thread Safety
//!
//!  * Wrap your type with [`Arc`] and then [`Mutex`] or [`RwLock`]
//!  * If you can avoid using [`Singleton::get_mut`] or get [`Singleton::try_get_mut`], or you know
//!  what you are doing
//!  * For Singleton instance in static context please use [`Mutex`]
//!
//! If none of this above not introduce it will definitely going to be data race
//!
//! ### Examples Concurrent Situation
//! ```
//! use std::{
//!     sync::{Arc, Mutex},
//!     thread::spawn,
//! };
//!
//! use singly::Singleton;
//!
//! struct Counter(i32);
//!
//! // Notice on the type
//! type ArcMutexCounter = Arc<Mutex<Counter>>;
//!
//! fn main() {
//!     let mut instance = Singleton::new();
//!     let counter = Arc::new(Mutex::new(Counter(0)));
//!     instance.set(counter);
//!
//!     let mut handles = vec![];
//!     for _ in 0..10 {
//!         let counter_clone: ArcMutexCounter = Arc::clone(instance.get::<ArcMutexCounter>());
//!         let handle = spawn(move || {
//!             let mut counter = counter_clone.lock().unwrap();
//!             (*counter).0 += 1;
//!         });
//!         handles.push(handle);
//!     }
//!
//!     let _ = handles
//!         .into_iter()
//!         .map(|handle| handle.join())
//!         .collect::<Result<Vec<_>, _>>();
//!
//!     let counter = instance.get::<ArcMutexCounter>().lock().unwrap().0;
//!     assert_eq!(counter, 10);
//! }
//! ```
//!
//! There is example on
//! [integration_test.rs](https://github.com/UnknownRori/singly-rs/blob/main/test/integration_test.rs)

#![no_std]

use core::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
};

use inner::Inner;

extern crate alloc;

mod inner;

#[derive(Debug)]
/// Base instance for Singleton storage
pub struct Singleton {
    /// Property to store any type of value in here
    /// It only allow single type every value
    /// If it insert with same type it will silently overwrite the old value
    storage: hashbrown::HashMap<TypeId, Inner>,
}

impl Singleton {
    /// Creates an empty Singleton Storage
    ///
    /// # Examples
    ///
    /// ```
    /// use singly::Singleton;
    ///
    /// let mut instance = Singleton::new();
    ///
    /// instance.set(32i32);
    /// instance.set(12f32);
    ///
    /// assert_eq!(instance.get::<i32>(), &32);
    /// assert_eq!(instance.get::<f32>(), &12f32);
    /// ```
    pub fn new() -> Self {
        Self {
            storage: hashbrown::HashMap::new(),
        }
    }

    /// Creates an empty Singleton Storage with specified capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use singly::Singleton;
    ///
    /// let mut instance = Singleton::with_capacity(2);
    ///
    /// instance.set(32i32);
    /// instance.set(12f32);
    ///
    /// assert_eq!(instance.get::<i32>(), &32);
    /// assert_eq!(instance.get::<f32>(), &12f32);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: hashbrown::HashMap::with_capacity(capacity),
        }
    }

    /// Store the data to [`Singleton`] storage
    /// Will silently overwrite old value if any
    pub fn set<T: Any>(&mut self, data: T) {
        self.storage.insert(TypeId::of::<T>(), Inner::new(data));
    }

    /// Get reference to data from global storage.
    /// Will return None if there is no data available with this type.
    pub fn try_get<T: Any>(&self) -> Option<&T> {
        self.storage
            .get(&TypeId::of::<T>())
            .and_then(|data| data.get().downcast_ref::<T>().map(|data| data.borrow()))
    }

    /// Get reference to data from global storage.
    ///
    /// ## Panic
    ///
    /// May panic if there is no data available with this type
    pub fn get<T: Any>(&self) -> &T {
        self.try_get::<T>().unwrap()
    }

    /// Get mutabler eference to data from global storage.
    /// Will return None if there is no data available with this type.
    pub fn try_get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.storage.get_mut(&TypeId::of::<T>()).and_then(|data| {
            data.get_mut()
                .downcast_mut::<T>()
                .map(|data| data.borrow_mut() as &mut T)
        })
    }

    /// Get mutable reference to data from global storage.
    ///
    /// ## Panic
    ///
    /// May panic if there is no data available with this type
    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.try_get_mut::<T>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_use() {
        let mut instance = Singleton::new();

        instance.set(32i32);
        instance.set(12f32);

        assert_eq!(instance.get::<i32>(), &32);
        assert_eq!(instance.get::<f32>(), &12f32);
    }

    #[test]
    fn set_ref_mut_value() {
        let mut instance = Singleton::new();
        instance.set(12i32);

        let a = instance.try_get_mut::<i32>().unwrap();
        *a = 13;

        assert_eq!(instance.get::<i32>(), &13);
    }
}
