use crate::sys::sync::mutex::Mutex;

/// we do not supported rwlock, so use mutex to simulate rwlock.
/// it's useful because so many code in std will use rwlock.
pub struct RwLock {
    inner: Mutex,
}

impl RwLock {
    #[inline]
    pub const fn new() -> RwLock {
        RwLock { inner: Mutex::new() }
    }

    #[inline]
    pub fn read(&self) {
        unsafe { self.inner.lock() };
    }

    #[inline]
    pub fn try_read(&self) -> bool {
        unsafe { self.inner.try_lock() }
    }

    #[inline]
    pub fn write(&self) {
        unsafe { self.inner.lock() };
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        unsafe { self.inner.try_lock() }
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        unsafe { self.inner.unlock() };
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        unsafe { self.inner.unlock() };
    }
}
