use crate::Garbler;

/// Trait for values that can be garbled
pub trait Garble: Sized {
    /// Output type after a garbling
    ///
    /// In most cases, this will be the same as the input type.
    type Output;

    /// Garble the data with the given garbler
    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler;
}
