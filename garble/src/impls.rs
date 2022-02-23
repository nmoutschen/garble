use crate::{Garble, Garbler, NoGarble};
use paste::paste;
use core::num;

/// Macro for creating Garble implementation for primitive types
/// 
/// While this could handle integer types, they are implemented separately
/// to also implement NonZero types.
macro_rules! impl_garble_primitive {
    ($primitive:ty) => {
        paste! {
            impl<'g> Garble<'g> for $primitive {
                type Output = $primitive;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g>,
                {
                    garbler.[<garble_ $primitive:lower>](self)
                }
            }
        }
    }
}
impl_garble_primitive! { bool }
impl_garble_primitive! { char }
impl_garble_primitive! { f32 }
impl_garble_primitive! { f64 }

/// Macro for creating Garble implementation for integer types
macro_rules! impl_garble_integer {
    ($primitive:ty, $nonzero:ty) => {
        paste! {
            impl<'g> Garble<'g> for $primitive {
                type Output = $primitive;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g>,
                {
                    garbler.[<garble_ $primitive:lower>](self)
                }
            }

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
impl_garble_integer! { u8, num::NonZeroU8 }
impl_garble_integer! { u16, num::NonZeroU16 }
impl_garble_integer! { u32, num::NonZeroU32 }
impl_garble_integer! { u64, num::NonZeroU64 }
impl_garble_integer! { u128, num::NonZeroU128 }
impl_garble_integer! { usize, num::NonZeroUsize }
impl_garble_integer! { i8, num::NonZeroI8 }
impl_garble_integer! { i16, num::NonZeroI16 }
impl_garble_integer! { i32, num::NonZeroI32 }
impl_garble_integer! { i64, num::NonZeroI64 }
impl_garble_integer! { i128, num::NonZeroI128 }
impl_garble_integer! { isize, num::NonZeroIsize }

///////////////////////////////////////////////////////////////////////////////
// Garble implementation for string types

macro_rules! impl_garble_str {
    ($type:ty) => {
        impl<'g> Garble<'g> for $type {
            type Output = String;

            fn garble<G>(self, garbler: &mut G) -> Self::Output
            where
                G: Garbler<'g>,
            {
                garbler.garble_str(self)
            }
        }
    }
}

impl_garble_str! { String }
impl_garble_str! { &str }

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

impl<'g, T> Garble<'g> for &'g T
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