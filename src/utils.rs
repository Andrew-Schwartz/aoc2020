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

pub fn lines(bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    bytes[0..bytes.len() - 1].split(|&b| b == b'\n')
}

macro_rules! day_input {
    (bytes $day:literal) => {
        include_bytes!(concat!("../input/2020/day", $day, ".txt"))
    };
    (str $day:literal) => {
        include_str!(concat!("../input/2020/day", $day, ".txt"))
    };
}