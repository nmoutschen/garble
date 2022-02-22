use paste::paste;

#[cfg(feature = "simple")]
mod simple;
#[cfg(feature = "simple")]
pub use simple::SimpleGarbler;

macro_rules! garble_func {
    ($($t:ty),*) => {
        $(
            paste! {
                /// Garble a(n) $t
                fn [<garble_ $t:lower>](&mut self, value: $t) -> $t;
            }
        )*
    }
}

/// Trait for something that can garble data
pub trait Garbler<'g>: Sized {
    garble_func!(
        bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char,
        String
    );
}
