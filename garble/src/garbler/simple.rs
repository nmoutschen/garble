use crate::Garbler;
use paste::paste;
use rand::prelude::*;

/// Simple implement of a randomizer [`Garbler`]
///
/// This will garble data randomly based on the given rate.
#[cfg_attr(docsrs, doc(cfg(feature = "simple")))]
#[derive(Debug)]
pub struct SimpleGarbler {
    rate: f64,
    rng: ThreadRng,
}

impl SimpleGarbler {
    /// Create a new [`SimpleGarbler`] with the given rate
    pub fn new(rate: f64) -> Self {
        Self {
            rate,
            rng: rand::thread_rng(),
        }
    }

    fn should_garble(&mut self) -> bool {
        self.rng.gen_bool(self.rate)
    }
}

macro_rules! impl_func {
    ($($t:ty => $v:expr),*) => {
        $(paste! {
            fn [<garble_ $t:lower>](&mut self, value: $t) -> $t {
                if self.should_garble() {
                    match self.rng.gen() {
                        v if v == value => $v(value),
                        v => v,
                    }
                } else {
                    value
                }
            }
        })*
    }
}
impl<'g> Garbler<'g> for SimpleGarbler {
    impl_func!(
        char => |v| std::char::from_u32(v as u32 + 1).unwrap_or('g'),
        u8 => |v| v + 1,
        u16 => |v| v + 1,
        u32 => |v| v + 1,
        u64 => |v| v + 1,
        u128 => |v| v + 1,
        usize => |v| v + 1,
        i8 => |v| v + 1,
        i16 => |v| v + 1,
        i32 => |v| v + 1,
        i64 => |v| v + 1,
        i128 => |v| v + 1,
        isize => |v| v + 1,
        f32 => |v: f32| v.powf(2.0),
        f64 => |v: f64| v.powf(2.0)
    );

    fn garble_bool(&mut self, value: bool) -> bool {
        self.should_garble() != value
    }

    fn garble_str<T>(&mut self, value: T) -> String
    where
        T: AsRef<str>,
    {
        value
            .as_ref()
            .chars()
            .map(|c| {
                if self.should_garble() {
                    self.rng.gen()
                } else {
                    c
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Garble, NoGarble};
    use paste::paste;

    macro_rules! test_case {
        ($t:ty => ($s:ident, $v:expr)) => {
            paste! {
                mod [<$t:lower _ $s>] {
                    use super::*;

                    #[test]
                    fn [<test_0pc>]() {
                        // GIVEN a SimpleGarbler with a rate of 0%
                        let mut garbler = SimpleGarbler::new(0.0);
                        // WHEN we garble a value
                        let value = $v.garble(&mut garbler);
                        // THEN the value should be the same as the original
                        assert_eq!(value, $v);
                    }

                    #[test]
                    fn [<test_100pc>]() {
                        // GIVEN a SimpleGarbler with a rate of 100%
                        let mut garbler = SimpleGarbler::new(1.0);
                        // WHEN we garble a value
                        let value = $v.garble(&mut garbler);
                        // THEN the value should be different
                        assert_ne!(value, $v);
                    }

                    #[test]
                    fn [<test_100pc_nogarble>]() {
                        // GIVEN a SimpleGarbler with a rate of 100%
                        let mut garbler = SimpleGarbler::new(100.0);
                        // WHEN we garble a non-garblable value
                        let value = NoGarble($v).garble(&mut garbler);
                        // THEN the value should be the same as the original
                        assert_eq!(value, $v);
                    }

                    #[test]
                    fn [<test_100pc_option>]() {
                        // GIVEN a SimpleGarbler with a rate of 100%
                        let mut garbler = SimpleGarbler::new(1.0);
                        // WHEN we garble an option
                        let value = Some($v).garble(&mut garbler);
                        // THEN the value should be different
                        if let Some(value) = value {
                            assert_ne!(value, $v);
                        } else {
                            assert!(false, "value should not be None");
                        }
                    }

                    #[test]
                    fn [<test_100pc_result>]() {
                        // GIVEN a SimpleGarbler with a rate of 100%
                        let mut garbler = SimpleGarbler::new(1.0);
                        // WHEN we garble a result
                        let value = Ok::<_, ()>($v).garble(&mut garbler);
                        // THEN the value should be different
                        if let Ok(value) = value {
                            assert_ne!(value, $v);
                        } else {
                            assert!(false, "value should not be Err");
                        }
                    }

                    #[test]
                    fn [<test_100pc_vec>]() {
                        // GIVEN a SimpleGarbler with a rate of 100%
                        let mut garbler = SimpleGarbler::new(1.0);
                        // WHEN we garble a vector
                        let value = vec![$v].garble(&mut garbler);
                        // THEN the value should be different
                        assert_ne!(value, vec![$v]);
                    }

                    #[test]
                    fn [<test_100pc_boxed>]() {
                        // GIVEN a SimpleGarbler with a rate of 100%
                        let mut garbler = SimpleGarbler::new(1.0);
                        // WHEN we garble a boxed value
                        let value = Box::new($v).garble(&mut garbler);
                        // THEN the value should be different
                        assert_ne!(value, $v);
                    }
                }
            }
        }
    }

    // Boolean tests
    test_case! { bool => (false, false) }
    test_case! { bool => (true, true) }

    // Unsigned integers
    test_case! { u8 => (min, u8::MIN) }
    test_case! { u8 => (max, u8::MAX) }
    test_case! { u16 => (min, u8::MIN) }
    test_case! { u16 => (max, u16::MAX) }
    test_case! { u32 => (min, u32::MIN) }
    test_case! { u32 => (max, u32::MAX) }
    test_case! { u64 => (min, u64::MIN) }
    test_case! { u64 => (max, u64::MAX) }
    test_case! { u128 => (min, u128::MIN) }
    test_case! { u128 => (max, u128::MAX) }
    test_case! { usize => (min, usize::MIN) }
    test_case! { usize => (max, usize::MAX) }

    // Signed integers
    test_case! { i8 => (min, i8::MIN) }
    test_case! { i8 => (max, i8::MAX) }
    test_case! { i8 => (zero, 0) }
    test_case! { i16 => (min, i16::MIN) }
    test_case! { i16 => (max, i16::MAX) }
    test_case! { i16 => (zero, 0) }
    test_case! { i32 => (min, i32::MIN) }
    test_case! { i32 => (max, i32::MAX) }
    test_case! { i32 => (zero, 0) }
    test_case! { i64 => (min, i64::MIN) }
    test_case! { i64 => (max, i64::MAX) }
    test_case! { i64 => (zero, 0) }
    test_case! { i128 => (min, i128::MIN) }
    test_case! { i128 => (max, i128::MAX) }
    test_case! { i128 => (zero, 0) }
    test_case! { isize => (min, isize::MIN) }
    test_case! { isize => (max, isize::MAX) }
    test_case! { isize => (zero, 0) }

    // Floating point numbers
    test_case! { f32 => (min, f32::MIN) }
    test_case! { f32 => (max, f32::MAX) }
    test_case! { f32 => (zero, 0.0) }
    test_case! { f64 => (min, f64::MIN) }
    test_case! { f64 => (max, f64::MAX) }
    test_case! { f64 => (zero, 0.0) }

    // Characters
    test_case! { char => (a, 'a') }
    test_case! { char => (carb, '🦀') }

    // String types
    test_case! { String => (short, String::from("hello, world")) }
    test_case! { str => (shprt, "hello, world") }
}
