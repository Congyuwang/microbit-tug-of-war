#![cfg_attr(not(test), no_std)]
use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};
use cortex_m::interrupt::CriticalSection;

/// Mutex ensure that data is accessed in interrupt free context.
///
/// The Mutex is designed for sharing data statically,
/// and allowing late value initialization.
pub struct Mutex<T>(UnsafeCell<MutexInner<T>>);

struct MutexInner<T> {
    state: MutexInnerState,
    value: MaybeUninit<T>,
}

enum MutexInnerState {
    Locked,
    Uinit,
    Unlock,
}

pub struct LockGaurd<'cs, T>(&'cs mut MutexInner<T>);

impl<T> Mutex<T> {
    /// Creates a new mutex.
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(MutexInner {
            state: MutexInnerState::Unlock,
            value: MaybeUninit::new(value),
        }))
    }

    /// Creates a new unit mutex.
    pub const fn new_uinit() -> Self {
        Self(UnsafeCell::new(MutexInner {
            state: MutexInnerState::Uinit,
            value: MaybeUninit::uninit(),
        }))
    }

    /// Value initialization.
    ///
    /// panic if already initalized.
    pub fn init<'cs>(&'cs self, _cs: &'cs CriticalSection, value: T) {
        let inner = unsafe { &mut *self.0.get() };
        if let MutexInnerState::Uinit = inner.state {
            inner.state = MutexInnerState::Unlock;
            inner.value = MaybeUninit::new(value);
        } else {
            panic!()
        }
    }

    /// Try to lock the mutex.
    pub fn try_lock<'cs>(&'cs self, _cs: &'cs CriticalSection) -> Option<LockGaurd<'cs, T>> {
        let inner = unsafe { &mut *self.0.get() };
        match inner.state {
            MutexInnerState::Uinit | MutexInnerState::Locked => None,
            MutexInnerState::Unlock => {
                inner.state = MutexInnerState::Locked;
                Some(LockGaurd(inner))
            }
        }
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.0.get() };
        if let MutexInnerState::Unlock | MutexInnerState::Locked = inner.state {
            unsafe { inner.value.assume_init_drop() }
        }
    }
}

impl<'cs, T> Deref for LockGaurd<'cs, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.value.assume_init_ref() }
    }
}

impl<'cs, T> DerefMut for LockGaurd<'cs, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.value.assume_init_mut() }
    }
}

impl<'cs, T> Drop for LockGaurd<'cs, T> {
    fn drop(&mut self) {
        self.0.state = MutexInnerState::Unlock;
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

#[cfg(test)]
mod test_mutex {
    use super::*;

    #[test]
    fn test_lock_unint() {
        let a = <Mutex<u32>>::new_uinit();
        let cs = unsafe { &CriticalSection::new() };
        assert!(matches!(a.try_lock(cs), None));
    }

    #[test]
    fn test_lock_init() {
        let a = <Mutex<u32>>::new_uinit();
        let cs = unsafe { &CriticalSection::new() };
        a.init(cs, 5);
        assert_eq!(*a.try_lock(cs).unwrap(), 5);
    }

    #[test]
    fn test_double_lock() {
        let a = <Mutex<u32>>::new_uinit();
        let cs = unsafe { &CriticalSection::new() };
        a.init(cs, 5);
        let mut val = a.try_lock(cs).unwrap();
        *val += 1;
        assert!(matches!(a.try_lock(cs), None));
    }

    #[test]
    fn test_lock_release() {
        let a = <Mutex<u32>>::new_uinit();
        let cs = unsafe { &CriticalSection::new() };
        a.init(cs, 5);
        {
            let mut val = a.try_lock(cs).unwrap();
            *val += 1;
        }
        assert_eq!(*a.try_lock(cs).unwrap(), 6);
    }
}
