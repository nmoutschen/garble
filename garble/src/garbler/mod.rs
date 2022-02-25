use crate::Garble;
use paste::paste;

#[cfg(feature = "simple")]
mod simple;
#[cfg(feature = "simple")]
pub use simple::SimpleGarbler;

macro_rules! garble_func {
    ($($t:ty),*) => {
        $(
            paste! {
                fn [<garble_ $t:lower>](&mut self, value: $t) -> $t;
            }
        )*
    }
}

/// Trait for something that can garble data
#[allow(missing_docs)]
pub trait Garbler<'g>: Sized {
    /// Convenience function for garbling a value
    ///
    /// This is equivalent to using [`Garble::garble`] and passing this `Garbler`.
    fn garble<T>(&mut self, value: T) -> T::Output
    where
        T: Garble<'g>,
    {
        value.garble(self)
    }

    garble_func!(
        // Other types
        bool, char, // Unsigned integers
        u8, u16, u32, u64, u128, usize, // Signed integers
        i8, i16, i32, i64, i128, isize, // Floats
        f32, f64
    );

    fn garble_str<T>(&mut self, value: T) -> String
    where
        T: AsRef<str>;
}
