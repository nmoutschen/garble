use crate::{Garble, NoGarble};

#[cfg(feature = "simple")]
mod simple;
#[cfg(feature = "simple")]
pub use simple::SimpleGarbler;

/// Trait for something that can garble data
pub trait Garbler<'g, T>: Sized
where
    T: Garble<'g>,
{
    /// Garble the data
    fn garble(&mut self, value: T) -> T::Output;
}

impl<'g, T, G> Garbler<'g, NoGarble<T>> for G {
    fn garble(&mut self, value: NoGarble<T>) -> T {
        value.0
    }
}
