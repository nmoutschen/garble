use crate::Garbler;
use rand::prelude::*;

/// Simple implement of a randomizer [`Garbler`]
///
/// This will garble data randomly based on the given rate.
#[cfg_attr(docsrs, doc(cfg(feature = "simple")))]
pub struct SimpleGarbler {
    rate: f64,
    rng: ThreadRng,
}

impl SimpleGarbler {
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

macro_rules! impl_value {
    ($($t:ty => $v:expr),*) => {
        $(
            impl<'g> Garbler<'g, $t> for SimpleGarbler {
                fn garble(&mut self, value: $t) -> $t {
                    if self.should_garble() {
                        match self.rng.gen() {
                            v if v == value => $v(value),
                            v => v,
                        }
                    } else {
                        value
                    }
                }
            }
        )*
    }
}
impl_value!(
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

impl<'g> Garbler<'g, bool> for SimpleGarbler {
    fn garble(&mut self, value: bool) -> bool {
        self.should_garble() != value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Garble, NoGarble};
    use paste::paste;

    macro_rules! test_values {
        ($($t:ty => ($s:ident, $v:expr)),*) => {
            $(paste! {
                mod [<$t _ $s>] {
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
                }
            })*
        }
    }

    test_values!(
        bool => (false, false),
        bool => (true, true),
        u8 => (min, u8::MIN),
        u8 => (max, u8::MAX),
        u16 => (min, u8::MIN),
        u16 => (max, u16::MAX),
        u32 => (min, u32::MIN),
        u32 => (max, u32::MAX),
        u64 => (min, u64::MIN),
        u64 => (max, u64::MAX),
        u128 => (min, u128::MIN),
        u128 => (max, u128::MAX),
        usize => (min, usize::MIN),
        usize => (max, usize::MAX),
        i8 => (min, i8::MIN),
        i8 => (max, i8::MAX),
        i8 => (zero, 0),
        i16 => (min, i16::MIN),
        i16 => (max, i16::MAX),
        i16 => (zero, 0),
        i32 => (min, i32::MIN),
        i32 => (max, i32::MAX),
        i32 => (zero, 0),
        i64 => (min, i64::MIN),
        i64 => (max, i64::MAX),
        i64 => (zero, 0),
        i128 => (min, i128::MIN),
        i128 => (max, i128::MAX),
        i128 => (zero, 0),
        isize => (min, isize::MIN),
        isize => (max, isize::MAX),
        isize => (zero, 0),
        f32 => (min, f32::MIN),
        f32 => (max, f32::MAX),
        f32 => (zero, 0.0),
        f64 => (min, f64::MIN),
        f64 => (max, f64::MAX),
        f64 => (zero, 0.0)
    );
}
