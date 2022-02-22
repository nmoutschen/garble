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
        ($($t:ty => $v:expr),*) => {
            $(paste! {
                #[test]
                fn [<test_0pc_ $t>]() {
                    let mut garbler = SimpleGarbler::new(0.0);
                    let value = $v.garble(&mut garbler);
                    assert_eq!(value, $v);
                }

                #[test]
                fn [<test_100pc_ $t>]() {
                    let mut garbler = SimpleGarbler::new(1.0);
                    let value = $v.garble(&mut garbler);
                    assert_ne!(value, $v);
                }

                #[test]
                fn [<test_0pc_nogarble_ $t>]() {
                    let mut garbler = SimpleGarbler::new(0.0);
                    let value = NoGarble($v).garble(&mut garbler);
                    assert_eq!(value, $v);
                }

                #[test]
                fn [<test_100pc_nogarble_ $t>]() {
                    let mut garbler = SimpleGarbler::new(100.0);
                    let value = NoGarble($v).garble(&mut garbler);
                    assert_eq!(value, $v);
                }
            })*
        }
    }

    test_values!(
        bool => false,
        u8 => 0,
        u16 => 0,
        u32 => 0,
        u64 => 0,
        u128 => 0,
        usize => 0,
        i8 => 0,
        i16 => 0,
        i32 => 0,
        i64 => 0,
        i128 => 0,
        isize => 0,
        f32 => 0.0,
        f64 => 0.0
    );
}
