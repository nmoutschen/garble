use crate::{Garble, Garbler};
use core::num;
use paste::paste;
use std::{collections, ffi, hash, marker, net, sync::atomic};

/// Macro for creating [`Garble`] implementations with a closure.
macro_rules! impl_garble {
    // Types with generics
    ($type:ty[$($generics:expr),+] => ($output:ty, $closure:tt)) => {
        paste! {
            impl<$($generics),+> Garble for $type<$($generics),+>
            where
                $(
                    $generics: Garble,
                )+
            {
                type Output = $output<$($generics::Output),+>;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler,
                {
                    ($closure)(self, garbler)
                }
            }
        }
    };
    // Types without generics
    ($type:ty => ($output:ty, $closure:tt)) => {
        paste! {
            impl Garble for $type {
                type Output = $output;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler,
                {
                    ($closure)(self, garbler)
                }
            }
        }
    };
}

/// Macro for creating [`Garble`] implementations for primitive types.
macro_rules! impl_garble_primitive {
    ($type:ty) => {
        impl_garble_primitive!($type => ($type, $type));
    };
    ($type:ty => ($output:ty, $func:ty)) => {
        impl_garble!($type => (
            $output,
            (paste! { |s: Self, g: &mut G|g.[<garble_ $func>](s) })
        ));
    };
}

/// Macro for creating [`Garble`] implementations for NonZero types
macro_rules! impl_garble_nonzero {
    ($primitive:ty, $nonzero:ty) => {
        impl_garble!($nonzero => (
            $nonzero,
            (paste! {
                |s: Self, g: &mut G| match g.[<garble_ $primitive>](s.get()) {
                    0 => $nonzero::new(1).unwrap(),
                    n => $nonzero::new(n).unwrap(),
                }
            })
        ));
    }
}

/// Macro for creating [`Garble`] implementations for Atomic types
macro_rules! impl_garble_atomic {
    ($primitive:ty, $atomic:ty) => {
        impl_garble!($atomic => (
            $atomic,
            (paste! {
                |s: Self, g: &mut G| $atomic::new(g.[<garble_ $primitive>](s.into_inner()))
            })
        ));
    }
}

/// Macro for creating [`Garble`] implementations for primitive types with
/// Atomic and NonZero options.
macro_rules! impl_garble_numeric {
    ($primitive:ty, NZ($nonzero:ty)) => {
        impl_garble_primitive! { $primitive => ($primitive, $primitive) }
        impl_garble_nonzero! { $primitive, $nonzero }
    };
    ($primitive:ty, AT($atomic:ty)) => {
        impl_garble_primitive! { $primitive => ($primitive, $primitive) }
        impl_garble_atomic! { $primitive, $atomic }
    };
    ($primitive:ty, NZ($nonzero:ty), AT($atomic:ty)) => {
        impl_garble_primitive! { $primitive => ($primitive, $primitive) }
        impl_garble_nonzero! { $primitive, $nonzero }
        impl_garble_atomic! { $primitive, $atomic }
    };
}

impl_garble_primitive!(char);
impl_garble_primitive!(f32);
impl_garble_primitive!(f64);
impl_garble_primitive!(String => (String, str));
impl_garble_primitive!(&str => (String, str));
impl_garble_numeric!(bool, AT(atomic::AtomicBool));
impl_garble_numeric!(u8, NZ(num::NonZeroU8), AT(atomic::AtomicU8));
impl_garble_numeric!(u16, NZ(num::NonZeroU16), AT(atomic::AtomicU16));
impl_garble_numeric!(u32, NZ(num::NonZeroU32), AT(atomic::AtomicU32));
impl_garble_numeric!(u64, NZ(num::NonZeroU64), AT(atomic::AtomicU64));
impl_garble_numeric!(u128, NZ(num::NonZeroU128));
impl_garble_numeric!(usize, NZ(num::NonZeroUsize), AT(atomic::AtomicUsize));
impl_garble_numeric!(i8, NZ(num::NonZeroI8), AT(atomic::AtomicI8));
impl_garble_numeric!(i16, NZ(num::NonZeroI16), AT(atomic::AtomicI16));
impl_garble_numeric!(i32, NZ(num::NonZeroI32), AT(atomic::AtomicI32));
impl_garble_numeric!(i64, NZ(num::NonZeroI64), AT(atomic::AtomicI64));
impl_garble_numeric!(i128, NZ(num::NonZeroI128));
impl_garble_numeric!(isize, NZ(num::NonZeroIsize), AT(atomic::AtomicIsize));

///////////////////////////////////////////////////////////////////////////////
// Garble implementation for empty types

impl Garble for () {
    type Output = ();

    fn garble<G>(self, _garbler: &mut G) -> Self::Output
    where
        G: Garbler,
    {
    }
}

impl<T> Garble for marker::PhantomData<T> {
    type Output = marker::PhantomData<T>;

    fn garble<G>(self, _garbler: &mut G) -> Self::Output
    where
        G: Garbler,
    {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for wrapping types

// Option<T>
impl_garble!(Option[T] => (
    Option,
    (|s: Self, g: &mut G| s.map(|v| v.garble(g)))
));

// Result<T, E>
impl_garble!(Result[T, E] => (
    Result,
    (|s: Self, g: &mut G| match s {
        Ok(v) => Ok(v.garble(g)),
        Err(e) => Err(e.garble(g)),
    })
));

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for arrays and slices

impl<T, const N: usize> Garble for [T; N]
where
    T: Garble,
{
    type Output = [T::Output; N];

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler,
    {
        self.map(|v| v.garble(garbler))
    }
}

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for tuples

macro_rules! impl_garble_tuple {
    ($($generics:tt),+) => {
        paste! {
            impl<$([<T $generics>]),+> Garble for ($([<T $generics>],)+)
            where
                $([<T $generics>]: Garble),+
            {
                type Output = ($([<T $generics>]::Output,)+);

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler,
                {
                    (
                        $(self.$generics.garble(garbler),)+
                    )
                }
            }
        }
    }
}
impl_garble_tuple!(0);
impl_garble_tuple!(0, 1);
impl_garble_tuple!(0, 1, 2);
impl_garble_tuple!(0, 1, 2, 3);
impl_garble_tuple!(0, 1, 2, 3, 4);
impl_garble_tuple!(0, 1, 2, 3, 4, 5);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_garble_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for sequences

macro_rules! impl_garble_sequence {
    ($type:ty) => {
        impl_garble!($type[T] => (
            $type,
            (|s: Self, g: &mut G| s.into_iter().map(|v| v.garble(g)).collect())
        ));
    }
}
impl_garble_sequence! { Vec }
impl_garble_sequence! { collections::VecDeque }
impl_garble_sequence! { collections::LinkedList }

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for maps

macro_rules! impl_garble_map {
    ($type:ty, $bounds:expr) => {
        paste! {
            impl<K, V> Garble for $type<K, V>
            where
                K: Garble,
                V: Garble,
                K::Output: $bounds,
            {
                type Output = $type<K::Output, V::Output>;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler,
                {
                    self.into_iter().map(|(k, v)| (k.garble(garbler), v.garble(garbler))).collect()
                }
            }
        }
    };
}
impl_garble_map!(collections::BTreeMap, Ord);
impl_garble_map!(collections::HashMap, hash::Hash + Eq);

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for sets

macro_rules! impl_garble_set {
    ($type:ty, $bounds:expr) => {
        paste! {
            impl<T> Garble for $type<T>
            where
                T: Garble,
                T::Output: $bounds,
            {
                type Output = $type<T::Output>;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler,
                {
                    self.into_iter().map(|v| v.garble(garbler)).collect()
                }
            }
        }
    };
}
impl_garble_set!(collections::BTreeSet, Ord);
impl_garble_set!(collections::HashSet, hash::Hash + Eq);
impl_garble_set!(collections::BinaryHeap, Ord);

///////////////////////////////////////////////////////////////////////////////
// Garble implementation for IP addresses

impl_garble!(net::Ipv4Addr => (
    net::Ipv4Addr,
    (|s: Self, garbler: &mut G| {
        let [a, b, c, d] = s.octets().garble(garbler);
        net::Ipv4Addr::new(a, b, c, d)
    })
));

impl_garble!(net::Ipv6Addr => (
    net::Ipv6Addr,
    (|s: Self, garbler: &mut G| {
        let [a, b, c, d, e, f, g, h] = s.segments().garble(garbler);
        net::Ipv6Addr::new(a, b, c, d, e, f, g, h)
    })
));

impl_garble!(net::IpAddr => (
    net::IpAddr,
    (|s: Self, garbler: &mut G| {
        match s {
            net::IpAddr::V4(addr) => net::IpAddr::V4(addr.garble(garbler)),
            net::IpAddr::V6(addr) => net::IpAddr::V6(addr.garble(garbler)),
        }
    })
));

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for sockets

impl_garble!(net::SocketAddrV4 => (
    net::SocketAddrV4,
    (|s: Self, garbler: &mut G| {
        net::SocketAddrV4::new(
            s.ip().garble(garbler),
            s.port().garble(garbler),
        )
    })
));

impl_garble!(net::SocketAddrV6 => (
    net::SocketAddrV6,
    (|s: Self, garbler: &mut G| {
        net::SocketAddrV6::new(
            s.ip().garble(garbler),
            s.port().garble(garbler),
            s.flowinfo(),
            s.scope_id(),
        )
    })
));

impl_garble!(net::SocketAddr => (
    net::SocketAddr,
    (|s: Self, garbler: &mut G| {
        match s {
            net::SocketAddr::V4(addr) => net::SocketAddr::V4(addr.garble(garbler)),
            net::SocketAddr::V6(addr) => net::SocketAddr::V6(addr.garble(garbler)),
        }
    })
));

///////////////////////////////////////////////////////////////////////////////
// Garble implementations for C strings

impl_garble!(ffi::CString => (
    ffi::CString,
    (|s: Self, garbler: &mut G| {
        let bytes = s.to_bytes().iter().map(|b| match b.garble(garbler) {
            // We cannot have nul bytes in a C string, so we replace them with
            // a question mark.
            0 => 0x3F,
            b => b,
        }).collect::<Vec<_>>();
        ffi::CString::new(bytes).unwrap()
    })
));

///////////////////////////////////////////////////////////////////////////////
// Garble implementation for borrowed values

impl<T> Garble for &T
where
    T: Garble + Clone,
{
    type Output = T::Output;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler,
    {
        self.clone().garble(garbler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct PassGarbler;

    macro_rules! impl_func {
        ($($t:ty),*) => {
            $(paste! {
                fn [<garble_ $t:lower>](&mut self, value: $t) -> $t {
                    value
                }
            })*
        }
    }

    impl Garbler for PassGarbler {
        impl_func! { char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool }

        fn garble_str<T>(&mut self, value: T) -> String
        where
            T: AsRef<str>,
        {
            value.as_ref().to_string()
        }
    }

    macro_rules! test_passthrough {
        ($name:ident, $value:expr) => {
            test_passthrough!($name, $value, $value);
        };
        ($name:ident, $orig:expr, $expect:expr) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $orig.garble(&mut garbler);
                    assert_eq!(garbled, $expect);
                }

                #[test]
                fn [<test_ $name _option>]() {
                    let mut garbler = PassGarbler;
                    let garbled = Some($orig).garble(&mut garbler);
                    assert_eq!(garbled, Some($expect));
                }

                #[test]
                fn [<test_ $name _ok>]() {
                    let mut garbler = PassGarbler;
                    let garbled = Ok::<_, ()>($orig).garble(&mut garbler);
                    assert_eq!(garbled, Ok($expect));
                }

                #[test]
                fn [<test_ $name _err>]() {
                    let mut garbler = PassGarbler;
                    let garbled = Err::<(), _>($orig).garble(&mut garbler);
                    assert_eq!(garbled, Err($expect));
                }

                #[test]
                fn [<test_ $name _vec>]() {
                    let mut garbler = PassGarbler;
                    let garbled = vec![$orig].garble(&mut garbler);
                    assert_eq!(garbled, vec![$expect]);
                }

                #[test]
                fn [<test_ $name _vec_option>]() {
                    let mut garbler = PassGarbler;
                    let garbled = vec![Some($orig)].garble(&mut garbler);
                    assert_eq!(garbled, vec![Some($expect)]);
                }

                #[test]
                fn [<test_ $name _option_vec>]() {
                    let mut garbler = PassGarbler;
                    let garbled = Some(vec![$orig]).garble(&mut garbler);
                    assert_eq!(garbled, Some(vec![$expect]));
                }
            }
        };
    }
    macro_rules! test_atomic {
        ($name:ident, $value:expr) => {
            paste! {
                #[test]
                fn [<test_atomic_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $value.garble(&mut garbler);
                    assert_eq!(garbled.into_inner(), $value.into_inner());
                }
            }
        };
    }
    macro_rules! test_nonzero {
        ($name:ident, $value:expr) => {
            paste! {
                #[test]
                fn [<test_nonzero_ $name>]() {
                    let mut garbler = PassGarbler;
                    let garbled = $value.garble(&mut garbler);
                    assert_eq!(garbled.get(), $value.get());
                }
            }
        };
    }

    // Character
    test_passthrough!(char, 'a');

    // Boolean
    test_passthrough!(bool, true);
    test_atomic!(bool, atomic::AtomicBool::new(true));

    // Unsigned integers
    test_passthrough!(u8, 0xFFu8);
    test_passthrough!(u16, 0xFFFFu16);
    test_passthrough!(u32, 0xFFFF_FFFFu32);
    test_passthrough!(u64, 0xFFFF_FFFF_FFFF_FFFFu64);
    test_passthrough!(u128, 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFu128);
    test_passthrough!(usize, 0xFFFF_FFFF_FFFF_FFFFusize);
    test_atomic!(u8, atomic::AtomicU8::new(0xFFu8));
    test_atomic!(u16, atomic::AtomicU16::new(0xFFFFu16));
    test_atomic!(u32, atomic::AtomicU32::new(0xFFFF_FFFFu32));
    test_atomic!(u64, atomic::AtomicU64::new(0xFFFF_FFFF_FFFF_FFFFu64));
    test_atomic!(usize, atomic::AtomicUsize::new(0xFFFF_FFFF_FFFF_FFFFusize));
    test_nonzero!(u8, num::NonZeroU8::new(0xFFu8).unwrap());
    test_nonzero!(u16, num::NonZeroU16::new(0xFFFFu16).unwrap());
    test_nonzero!(u32, num::NonZeroU32::new(0xFFFF_FFFFu32).unwrap());
    test_nonzero!(u64, num::NonZeroU64::new(0xFFFF_FFFF_FFFF_FFFFu64).unwrap());
    test_nonzero!(
        u128,
        num::NonZeroU128::new(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFu128).unwrap()
    );
    test_nonzero!(
        usize,
        num::NonZeroUsize::new(0xFFFF_FFFF_FFFF_FFFFusize).unwrap()
    );

    // Signed integers
    test_passthrough!(i8, -0x7Fi8);
    test_passthrough!(i16, -0x7FFFi16);
    test_passthrough!(i32, -0x7FFF_FFFFi32);
    test_passthrough!(i64, -0x7FFF_FFFF_FFFF_FFFFi64);
    test_passthrough!(i128, -0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFi128);
    test_passthrough!(isize, -0x7FFF_FFFF_FFFF_FFFFisize);
    test_atomic!(i8, atomic::AtomicI8::new(-0x7Fi8));
    test_atomic!(i16, atomic::AtomicI16::new(-0x7FFFi16));
    test_atomic!(i32, atomic::AtomicI32::new(-0x7FFF_FFFFi32));
    test_atomic!(i64, atomic::AtomicI64::new(-0x7FFF_FFFF_FFFF_FFFFi64));
    test_atomic!(isize, atomic::AtomicIsize::new(-0x7FFF_FFFF_FFFF_FFFFisize));
    test_nonzero!(i8, num::NonZeroI8::new(-0x7Fi8).unwrap());
    test_nonzero!(i16, num::NonZeroI16::new(-0x7FFFi16).unwrap());
    test_nonzero!(i32, num::NonZeroI32::new(-0x7FFF_FFFFi32).unwrap());
    test_nonzero!(
        i64,
        num::NonZeroI64::new(-0x7FFF_FFFF_FFFF_FFFFi64).unwrap()
    );
    test_nonzero!(
        i128,
        num::NonZeroI128::new(-0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFFi128).unwrap()
    );
    test_nonzero!(
        isize,
        num::NonZeroIsize::new(-0x7FFF_FFFF_FFFF_FFFFisize).unwrap()
    );

    // Floating point numbers
    test_passthrough!(f32, 0.0_f32);
    test_passthrough!(f64, 0.0_f64);

    // Strings
    test_passthrough!(str, "Hello, world!", String::from("Hello, world!"));
    test_passthrough!(
        string,
        String::from("Hello, world!"),
        String::from("Hello, world!")
    );
    test_passthrough!(
        borrowed_string,
        &String::from("Hello, world!"),
        String::from("Hello, world!")
    );

    // Tuples
    test_passthrough!(tuple1, ("a",), ("a".to_string(),));
    test_passthrough!(tuple2, (2, 'a'));
    test_passthrough!(tuple3, (3, 'a', 1.5f32));

    // Bytes
    test_passthrough!(bytes, b"Hello, world!", b"Hello, world!".to_owned());
    test_passthrough!(bytes_owned, b"Hello, world!".to_owned());

    // IP Addresses
    test_passthrough!(ipv4, net::Ipv4Addr::new(127, 0, 0, 1));
    test_passthrough!(ipv6, net::Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0));

    // Sockets
    test_passthrough!(
        socket_ipv4,
        net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 8080)
    );
    test_passthrough!(
        socket_ipv6,
        net::SocketAddr::new(
            net::IpAddr::V6(net::Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0)),
            8080
        )
    );

    // CStrings
    test_passthrough!(cstring, ffi::CString::new("Hello, world!").unwrap());
    test_passthrough!(
        borrowed_cstring,
        &ffi::CString::new("Hello, world!").unwrap(),
        ffi::CString::new("Hello, world!").unwrap()
    );
}
