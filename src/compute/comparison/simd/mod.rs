use crate::types::NativeType;

/// [`NativeType`] that supports a representation of 8 lanes
pub trait Simd8: NativeType {
    type Simd: Simd8Lanes<Self>;
}

pub trait Simd8Lanes<T>: Copy {
    fn from_chunk(v: &[T]) -> Self;
    fn from_incomplete_chunk(v: &[T], remaining: T) -> Self;
    fn eq(self, other: Self) -> u8;
    fn neq(self, other: Self) -> u8;
    fn lt_eq(self, other: Self) -> u8;
    fn lt(self, other: Self) -> u8;
    fn gt(self, other: Self) -> u8;
    fn gt_eq(self, other: Self) -> u8;
}

#[inline]
pub(super) fn set<T: Copy, F: Fn(T, T) -> bool>(lhs: [T; 8], rhs: [T; 8], op: F) -> u8 {
    let mut byte = 0u8;
    lhs.iter()
        .zip(rhs.iter())
        .enumerate()
        .for_each(|(i, (lhs, rhs))| {
            byte |= if op(*lhs, *rhs) { 1 << i } else { 0 };
        });
    byte
}

macro_rules! simd8_native {
    ($type:ty) => {
        impl Simd8 for $type {
            type Simd = [$type; 8];
        }

        impl Simd8Lanes<$type> for [$type; 8] {
            #[inline]
            fn from_chunk(v: &[$type]) -> Self {
                v.try_into().unwrap()
            }

            #[inline]
            fn from_incomplete_chunk(v: &[$type], remaining: $type) -> Self {
                let mut a = [remaining; 8];
                a.iter_mut().zip(v.iter()).for_each(|(a, b)| *a = *b);
                a
            }

            #[inline]
            fn eq(self, other: Self) -> u8 {
                set(self, other, |x, y| x == y)
            }

            #[inline]
            fn neq(self, other: Self) -> u8 {
                #[allow(clippy::float_cmp)]
                set(self, other, |x, y| x != y)
            }

            #[inline]
            fn lt_eq(self, other: Self) -> u8 {
                set(self, other, |x, y| x <= y)
            }

            #[inline]
            fn lt(self, other: Self) -> u8 {
                set(self, other, |x, y| x < y)
            }

            #[inline]
            fn gt_eq(self, other: Self) -> u8 {
                set(self, other, |x, y| x >= y)
            }

            #[inline]
            fn gt(self, other: Self) -> u8 {
                set(self, other, |x, y| x > y)
            }
        }
    };
}

#[cfg(not(feature = "simd"))]
mod native;
#[cfg(not(feature = "simd"))]
pub use native::*;
#[cfg(feature = "simd")]
mod packed;
#[cfg(feature = "simd")]
pub use packed::*;
