#[repr(C)]
pub struct SharedPtr<T> {
    data: *mut T,
    shared_weak_count: *mut ()
}

impl<T> SharedPtr<T> {
    pub fn null() -> Self {
        Self {
            data: std::ptr::null_mut(),
            shared_weak_count: std::ptr::null_mut(),
        }
    }
    pub fn is_null(&self) -> bool {
        self.data.is_null() || self.shared_weak_count.is_null()
    }
}

impl<T> std::ops::Deref for SharedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> std::ops::DerefMut for SharedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data }
    }
}