/// Wrapper for a value that shouldn't be garbled
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoGarble<T>(pub(crate) T);

impl<T> std::ops::Deref for NoGarble<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for NoGarble<T> {
    fn from(t: T) -> Self {
        NoGarble(t)
    }
}
