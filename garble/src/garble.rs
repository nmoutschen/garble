use crate::Garbler;

/// Trait for values that can be garbled
pub trait Garble<'g>: Sized {
    type Output;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g, Self>;
}

macro_rules! impl_garble {
    ($($t:ty),*) => {
        $(
            impl<'g> Garble<'g> for $t {
                type Output = $t;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g, Self>,
                {
                    garbler.garble(self)
                }
            }
        )*
    };
}
impl_garble!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, String
);

/// Wrapper for a value that shouldn't be garbled
pub struct NoGarble<T>(pub T);

impl<T> std::ops::Deref for NoGarble<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'g, T> Garble<'g> for NoGarble<T> {
    type Output = T;

    fn garble<G>(self, _garble: &mut G) -> Self::Output
    where
        G: Garbler<'g, Self>,
    {
        self.0
    }
}
