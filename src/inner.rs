//! This is where the magic

use core::{any::Any, cell::UnsafeCell, ptr::NonNull};

use alloc::boxed::Box;

#[derive(Debug)]
/// Very not safe wrapper to allow bypass safety thread safety
pub struct Inner {
    inner: UnsafeCell<NonNull<dyn Any>>,
}

impl Inner {
    /// Creating and instance of Inner
    ///
    /// ## Panic
    ///
    /// Will if the pointer is not valid
    pub fn new<T: Any>(val: T) -> Self {
        Self {
            inner: UnsafeCell::new(NonNull::new(Box::into_raw(Box::new(val))).unwrap()),
        }
    }

    /// Get the reference of content of Inner
    ///
    /// # Panic
    ///
    /// Not sure what will make this thing panic
    pub fn get(&self) -> &dyn Any {
        unsafe { self.inner.get().as_ref().map(|ptr| &*ptr.as_ptr()).unwrap() }
    }

    /// Get the mutable reference of content of Inner
    ///
    /// # Panic
    ///
    /// Not sure what will make this thing panic
    pub fn get_mut(&self) -> &mut dyn Any {
        unsafe {
            self.inner
                .get()
                .as_mut()
                .map(|ptr| &mut *ptr.as_ptr())
                .unwrap()
        }
    }
}

unsafe impl Sync for Inner {}
unsafe impl Send for Inner {}

#[cfg(test)]
mod test {
    use super::Inner;

    #[test]
    fn test_if_value_is_correct() {
        let inner = Inner::new(10);

        assert_eq!(inner.get().downcast_ref::<i32>().unwrap(), &10);
        assert_eq!(inner.get_mut().downcast_ref::<i32>().unwrap(), &10);
    }
}
