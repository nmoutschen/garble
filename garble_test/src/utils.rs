use garble::Garbler;
use paste::paste;

pub(crate) struct ZeroGarbler;

macro_rules! impl_func {
    ($t:ty, $v:expr) => {
        paste! {
            fn [<garble_ $t:lower>](&mut self, _value: $t) -> $t {
                $v
            }
        }
    };
}

impl Garbler for ZeroGarbler {
    impl_func!(bool, false);
    impl_func!(u8, 0);
    impl_func!(u16, 0);
    impl_func!(u32, 0);
    impl_func!(u64, 0);
    impl_func!(u128, 0);
    impl_func!(usize, 0);
    impl_func!(i8, 0);
    impl_func!(i16, 0);
    impl_func!(i32, 0);
    impl_func!(i64, 0);
    impl_func!(i128, 0);
    impl_func!(isize, 0);
    impl_func!(f32, 0.0);
    impl_func!(f64, 0.0);
    impl_func!(char, ' ');

    fn garble_str<T>(&mut self, _value: T) -> String
    where
        T: AsRef<str>,
    {
        String::new()
    }
}
