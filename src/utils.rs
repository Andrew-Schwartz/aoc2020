use std::hint::unreachable_unchecked;

pub trait UnwrapUnchecked<T> {
    unsafe fn unwrap_unchecked(self) -> T;
}

impl<T> UnwrapUnchecked<T> for Option<T> {
    unsafe fn unwrap_unchecked(self) -> T {
        if let Some(some) = self {
            some
        } else {
            unreachable_unchecked()
        }
    }
}

impl<T, E> UnwrapUnchecked<T> for Result<T, E> {
    unsafe fn unwrap_unchecked(self) -> T {
        if let Ok(ok) = self {
            ok
        } else {
            unreachable_unchecked()
        }
    }
}