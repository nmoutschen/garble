use crate::{Garbler, NoGarble};
use paste::paste;

// TODO:
// - tuples
// - non-zero
// - atomics
// - CString
// - CStr
// - &str
// - &[u8]
// - PhantomData
// - BinaryHeap
// - BTreeSet
// - BTreeMap
// - HashSet
// - HashMap
// - LinkedList
// - VecDeque
// - IpAddr
// - Ipv4Addr
// - Ipv6Addr
// - SocketAddr
// - SocketAddrV4
// - SocketAddrV6
// - &Path
// - PathBuf
// - OsString
// - OsStr
// - Cow
// and probably more
//
// see https://docs.rs/serde/latest/serde/trait.Deserialize.html for inspiration

/// Trait for values that can be garbled
pub trait Garble<'g>: Sized {
    /// Output type after a garbling
    ///
    /// In most cases, this will be the same as the input type.
    type Output;

    /// Garble the data with the given garbler
    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>;
}

macro_rules! impl_type {
    ($($t:ty),*) => {
        $(paste! { impl<'g> Garble<'g> for $t {
                type Output = $t;

                fn garble<G>(self, garbler: &mut G) -> Self::Output
                where
                    G: Garbler<'g>,
                {
                    garbler.[<garble_ $t:lower>](self)
                }
            }
        })*
    };
}
impl_type!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, String
);

impl<'g, T> Garble<'g> for NoGarble<T> {
    type Output = T;

    fn garble<G>(self, _garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.0
    }
}

impl<'g, T> Garble<'g> for Option<T>
where
    T: Garble<'g>,
{
    type Output = Option<T::Output>;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.map(|v| v.garble(garbler))
    }
}

impl<'g, T, E> Garble<'g> for Result<T, E>
where
    T: Garble<'g>,
    E: Garble<'g>,
{
    type Output = Result<T::Output, E::Output>;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        match self {
            Ok(v) => Ok(v.garble(garbler)),
            Err(e) => Err(e.garble(garbler)),
        }
    }
}

impl<'g, T> Garble<'g> for Vec<T>
where
    T: Garble<'g>,
{
    type Output = Vec<T::Output>;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.into_iter().map(|v| v.garble(garbler)).collect()
    }
}

impl<'g, T, const N: usize> Garble<'g> for [T; N]
where
    T: Garble<'g>,
{
    type Output = [T::Output; N];

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.map(|v| v.garble(garbler))
    }
}

impl<'g, T> Garble<'g> for &'g T
where
    T: Garble<'g> + Clone,
{
    type Output = T::Output;

    fn garble<G>(self, garbler: &mut G) -> Self::Output
    where
        G: Garbler<'g>,
    {
        self.clone().garble(garbler)
    }
}