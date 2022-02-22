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
    fn garble(&mut self, value: T) -> T;
}

impl<'g, T, G> Garbler<'g, NoGarble<T>> for G {
    fn garble(&mut self, value: NoGarble<T>) -> NoGarble<T> {
        value
    }
}