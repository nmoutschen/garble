use crate::{Garble, Garbler, NoGarble};
use paste::paste;
use core::num;
use std::sync::atomic;

/// Macro for creating [`Garble`] implementations for a type using a [`Garbler`] function
/// directly
macro_rules! impl_garble_base {
    ($type:expr => ($output:expr, $func:expr)) => {
        paste! {
            impl<'g> Garble<'g> for $type {
                type Output = $output;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g>,
                {
                    garbler.[<garble_ $func>](self)
                }
            }
        }
    }
}

/// Macro for creating [`Garble`] implementations for NonZero types
macro_rules! impl_garble_nonzero {
    ($primitive:expr, $nonzero:expr) => {
        paste! {
            impl<'g> Garble<'g> for $nonzero {
                type Output = $nonzero;
                
                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g>,
                {
                    match garbler.[<garble_ $primitive:lower>](self.get()) {
                        0 => $nonzero::new(1).unwrap(),
                        v => $nonzero::new(v).unwrap(),
                    }
                }
            }
        }
    }
}

/// Macro for creating [`Garble`] implementations for Atomic types
macro_rules! impl_garble_atomic {
    ($primitive:expr, $atomic:expr) => {
        paste! {
            impl<'g> Garble<'g> for $atomic {
                type Output = $atomic;
            
                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g>,
                {
                    $atomic::new(garbler.[<garble_ $primitive>](self.into_inner()))
                }
            }
        }
    }
}

/// Macro for creating [`Garble`] implementations
macro_rules! impl_garble {
    ($type:expr => ($output:expr, $func:expr)) => {
        impl_garble_base! { $type => ($output, $func) }
    };

    ($primitive:expr) => {
        impl_garble_base! { $primitive => ($primitive, $primitive) }
    };
    ($primitive:expr, NZ($nonzero:expr)) => {
        impl_garble_base! { $primitive => ($primitive, $primitive) }
        impl_garble_nonzero! { $primitive, $nonzero }
    };
    ($primitive:expr, AT($atomic:expr)) => {
        impl_garble_base! { $primitive => ($primitive, $primitive) }
        impl_garble_atomic! { $primitive, $atomic }
    };
    ($primitive:expr, NZ($nonzero:expr), AT($atomic:expr)) => {
        impl_garble_base! { $primitive => ($primitive, $primitive) }
        impl_garble_nonzero! { $primitive, $nonzero }
        impl_garble_atomic! { $primitive, $atomic }
    };
}

impl_garble! { char }
impl_garble! { f32 }
impl_garble! { f64 }
impl_garble! { bool, AT(atomic::AtomicBool) }
impl_garble! { u8, NZ(num::NonZeroU8), AT(atomic::AtomicU8) }
impl_garble! { u16, NZ(num::NonZeroU16), AT(atomic::AtomicU16) }
impl_garble! { u32, NZ(num::NonZeroU32), AT(atomic::AtomicU32) }
impl_garble! { u64, NZ(num::NonZeroU64), AT(atomic::AtomicU64) }
impl_garble! { u128, NZ(num::NonZeroU128) }
impl_garble! { usize, NZ(num::NonZeroUsize), AT(atomic::AtomicUsize) }
impl_garble! { i8, NZ(num::NonZeroI8), AT(atomic::AtomicI8) }
impl_garble! { i16, NZ(num::NonZeroI16), AT(atomic::AtomicI16) }
impl_garble! { i32, NZ(num::NonZeroI32), AT(atomic::AtomicI32) }
impl_garble! { i64, NZ(num::NonZeroI64), AT(atomic::AtomicI64) }
impl_garble! { i128, NZ(num::NonZeroI128) }
impl_garble! { isize, NZ(num::NonZeroIsize), AT(atomic::AtomicIsize) }

impl_garble! { String => (String, str) }
impl_garble! { &str => (String, str) }

///////////////////////////////////////////////////////////////////////////////
// Garble implementation for ()

impl<'g> Garble<'g> for () {
    type Output = ();

    fn garble<G>(self, _garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        ()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for wrapping types

/// Garble implementation for [`NoGarble`]
/// 
/// For NoGarble, we can ignore if the value is [`Garble`] or not, as it will
/// never be garbled.
impl<'g, T> Garble<'g> for NoGarble<T> {
    type Output = T;

    fn garble<G>(self, _garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.0
    }
}

impl<'g, T> Garble<'g> for Option<T>
where
    T: Garble<'g>,
{
    type Output = Option<T::Output>;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.map(|v| v.garble(garbler))
    }
}

impl<'g, T, E> Garble<'g> for Result<T, E>
where
    T: Garble<'g>,
    E: Garble<'g>,
{
    type Output = Result<T::Output, E::Output>;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        match self {
            Ok(v) => Ok(v.garble(garbler)),
            Err(e) => Err(e.garble(garbler)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for arrays, vectors, and slices

impl<'g, T> Garble<'g> for Vec<T>
where
    T: Garble<'g>,
{
    type Output = Vec<T::Output>;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.into_iter().map(|v| v.garble(garbler)).collect()
    }
}

impl<'g, T, const N: usize> Garble<'g> for [T; N]
where
    T: Garble<'g>,
{
    type Output = [T::Output; N];

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.map(|v| v.garble(garbler))
    }
}

///////////////////////////////////////////////////////////////////////////////
// Garble implementation for borrowed values

impl<'g, T> Garble<'g> for &T
where
    T: Garble<'g> + Clone,
{
    type Output = T::Output;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.clone().garble(garbler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct PassGarbler;

    macro_rules! impl_func {
        ($t:ty) => {
            paste! {
                fn [<garble_ $t:lower>](&mut self, value: $t) -> $t {
                    value
                }
            }
        }
    }

    impl<'g> Garbler<'g> for PassGarbler {
        impl_func! { char }
        impl_func! { u8 }
        impl_func! { u16 }
        impl_func! { u32 }
        impl_func! { u64 }
        impl_func! { u128 }
        impl_func! { usize }
        impl_func! { i8 }
        impl_func! { i16 }
        impl_func! { i32 }
        impl_func! { i64 }
        impl_func! { i128 }
        impl_func! { isize }
        impl_func! { f32 }
        impl_func! { f64 }
        impl_func! { bool }

        fn garble_str<T>(&mut self, value: T) -> String
        where
            T: AsRef<str>,
        {
            value.as_ref().to_string()
        }
    }

    macro_rules! test_passthrough {
        ($name:ident, $value:expr) => {
            paste! {
                #[test]
                fn [<test_passthrough_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $value.garble(&mut garbler);
                    assert_eq!(garbled, $value);
                }
            }
        };
        ($name:ident, $orig:expr, $expect:expr) => {
            paste! {
                #[test]
                fn [<test_passthrough_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $orig.garble(&mut garbler);
                    assert_eq!(garbled, $expect);
                }
            }
        };
    }
    macro_rules! test_atomic {
        ($name:ident, $value:expr) => {
            paste! {
                #[test]
                fn [<test_atomic_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $value.garble(&mut garbler);
                    assert_eq!(garbled.into_inner(), $value.into_inner());
                }
            }
        }
    }
    macro_rules! test_nonzero {
        ($name:ident, $value:expr) => {
            paste! {
                #[test]
                fn [<test_nonzero_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $value.garble(&mut garbler);
                    assert_eq!(garbled.get(), $value.get());
                }
            }
        }
    }

    // Character
    test_passthrough! { char, 'a' }

    // Boolean
    test_passthrough! { bool, true }
    test_atomic! { bool, atomic::AtomicBool::new(true) }

    // Unsigned integers
    test_passthrough! { u8, 0xFFu8 }
    test_passthrough! { u16, 0xFFFFu16 }
    test_passthrough! { u32, 0xFFFF_FFFFu32 }
    test_passthrough! { u64, 0xFFFF_FFFF_FFFF_FFFFu64 }
    test_passthrough! { u128, 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFu128 }
    test_passthrough! { usize, 0xFFFF_FFFF_FFFF_FFFFusize }
    test_atomic! { u8, atomic::AtomicU8::new(0xFFu8) }
    test_atomic! { u16, atomic::AtomicU16::new(0xFFFFu16) }
    test_atomic! { u32, atomic::AtomicU32::new(0xFFFF_FFFFu32) }
    test_atomic! { u64, atomic::AtomicU64::new(0xFFFF_FFFF_FFFF_FFFFu64) }
    test_atomic! { usize, atomic::AtomicUsize::new(0xFFFF_FFFF_FFFF_FFFFusize) }
    test_nonzero! { u8, num::NonZeroU8::new(0xFFu8).unwrap() }
    test_nonzero! { u16, num::NonZeroU16::new(0xFFFFu16).unwrap() }
    test_nonzero! { u32, num::NonZeroU32::new(0xFFFF_FFFFu32).unwrap() }
    test_nonzero! { u64, num::NonZeroU64::new(0xFFFF_FFFF_FFFF_FFFFu64).unwrap() }
    test_nonzero! { u128, num::NonZeroU128::new(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFu128).unwrap() }
    test_nonzero! { usize, num::NonZeroUsize::new(0xFFFF_FFFF_FFFF_FFFFusize).unwrap() }

    // Signed integers
    test_passthrough! { i8, -0x7Fi8 }
    test_passthrough! { i16, -0x7FFFi16 }
    test_passthrough! { i32, -0x7FFF_FFFFi32 }
    test_passthrough! { i64, -0x7FFF_FFFF_FFFF_FFFFi64 }
    test_passthrough! { i128, -0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFi128 }
    test_passthrough! { isize, -0x7FFF_FFFF_FFFF_FFFFisize }
    test_atomic! { i8, atomic::AtomicI8::new(-0x7Fi8) }
    test_atomic! { i16, atomic::AtomicI16::new(-0x7FFFi16) }
    test_atomic! { i32, atomic::AtomicI32::new(-0x7FFF_FFFFi32) }
    test_atomic! { i64, atomic::AtomicI64::new(-0x7FFF_FFFF_FFFF_FFFFi64) }
    test_atomic! { isize, atomic::AtomicIsize::new(-0x7FFF_FFFF_FFFF_FFFFisize) }
    test_nonzero! { i8, num::NonZeroI8::new(-0x7Fi8).unwrap() }
    test_nonzero! { i16, num::NonZeroI16::new(-0x7FFFi16).unwrap() }
    test_nonzero! { i32, num::NonZeroI32::new(-0x7FFF_FFFFi32).unwrap() }
    test_nonzero! { i64, num::NonZeroI64::new(-0x7FFF_FFFF_FFFF_FFFFi64).unwrap() }
    test_nonzero! { i128, num::NonZeroI128::new(-0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFi128).unwrap() }
    test_nonzero! { isize, num::NonZeroIsize::new(-0x7FFF_FFFF_FFFF_FFFFisize).unwrap() }


    // Floating point numbers
    test_passthrough! { f32, 0.0_f32 }
    test_passthrough! { f64, 0.0_f64 }

    // Strings
    test_passthrough! { str, "Hello, world!", String::from("Hello, world!") }
    test_passthrough! { string, String::from("Hello, world!"), String::from("Hello, world!") }
    test_passthrough! { borrowed_string, &String::from("Hello, world!"), String::from("Hello, world!") }

    // Bytes
    test_passthrough! { bytes, b"Hello, world!", b"Hello, world!".to_owned() }
    test_passthrough! { bytes_owned, b"Hello, world!".to_owned() }
}