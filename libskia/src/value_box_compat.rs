use std::any::Any;
use std::mem;

use value_box::{BorrowedPtr, OwnedPtr, Result, ReturnBoxerResult};

pub trait PtrFallback<R> {
    fn into_fallback(self) -> R;
}

impl<R> PtrFallback<R> for R {
    fn into_fallback(self) -> R {
        self
    }
}

impl<T: Any, U> PtrFallback<OwnedPtr<T>> for *mut U {
    fn into_fallback(self) -> OwnedPtr<T> {
        OwnedPtr::null()
    }
}

pub trait BorrowedPtrCompat<T: Any> {
    fn has_value(&self) -> bool;
    fn with_not_null<R: Any, F>(&self, op: F)
    where
        F: FnOnce(&mut T) -> R;
    fn with_not_null_return<R: Any, D, F>(&self, default: D, op: F) -> R
    where
        D: PtrFallback<R>,
        F: FnOnce(&mut T) -> R;
    fn with_not_null_value<R: Any, F>(&self, op: F)
    where
        F: FnOnce(T) -> R,
        T: Clone;
    fn with_not_null_value_return<R: Any, D, F>(&self, default: D, op: F) -> R
    where
        D: PtrFallback<R>,
        F: FnOnce(T) -> R,
        T: Clone;
    fn with_value<R: Any, FN, FS>(&self, none: FN, some: FS) -> R
    where
        FN: FnOnce() -> R,
        FS: FnOnce(T) -> R,
        T: Clone;
}

impl<T: Any> BorrowedPtrCompat<T> for BorrowedPtr<T> {
    fn has_value(&self) -> bool {
        !self.is_null()
    }

    fn with_not_null<R: Any, F>(&self, op: F)
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut ptr = unsafe { BorrowedPtr::from_raw(self.as_raw()) };
        ptr.with_mut_ok(op).log();
    }

    fn with_not_null_return<R: Any, D, F>(&self, default: D, op: F) -> R
    where
        D: PtrFallback<R>,
        F: FnOnce(&mut T) -> R,
    {
        let mut ptr = unsafe { BorrowedPtr::from_raw(self.as_raw()) };
        ptr.with_mut_ok(op).or_log(default.into_fallback())
    }

    fn with_not_null_value<R: Any, F>(&self, op: F)
    where
        F: FnOnce(T) -> R,
        T: Clone,
    {
        self.with_clone_ok(op).log();
    }

    fn with_not_null_value_return<R: Any, D, F>(&self, default: D, op: F) -> R
    where
        D: PtrFallback<R>,
        F: FnOnce(T) -> R,
        T: Clone,
    {
        self.with_clone_ok(op).or_log(default.into_fallback())
    }

    fn with_value<R: Any, FN, FS>(&self, none: FN, some: FS) -> R
    where
        FN: FnOnce() -> R,
        FS: FnOnce(T) -> R,
        T: Clone,
    {
        self.with_option_ref(|value| {
            Ok(match value {
                Some(value) => some(value.clone()),
                None => none(),
            })
        })
        .expect("with_option_ref does not fail")
    }
}

pub trait OwnedPtrCompat<T: Any> {
    fn has_value(&self) -> bool;
    fn into_raw(self) -> OwnedPtr<T>;
    fn release(&mut self);
    fn take_value(&mut self) -> Result<T>;
}

impl<T: Any> OwnedPtrCompat<T> for OwnedPtr<T> {
    fn has_value(&self) -> bool {
        !self.is_null()
    }

    fn into_raw(self) -> OwnedPtr<T> {
        self
    }

    fn release(&mut self) {
        let ptr = mem::take(self);
        drop(ptr);
    }

    fn take_value(&mut self) -> Result<T> {
        mem::take(self).with_value_ok(|value| value)
    }
}

pub trait OwnedPtrResultCompat<T: Any> {
    fn into_raw(self) -> OwnedPtr<T>;
}

impl<T: Any> OwnedPtrResultCompat<T> for Result<OwnedPtr<T>> {
    fn into_raw(self) -> OwnedPtr<T> {
        self.or_log(OwnedPtr::null())
    }
}

impl<T: Any> OwnedPtrResultCompat<T> for Result<Option<OwnedPtr<T>>> {
    fn into_raw(self) -> OwnedPtr<T> {
        self.or_log(None).unwrap_or_default()
    }
}
