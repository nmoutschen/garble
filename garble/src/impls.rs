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