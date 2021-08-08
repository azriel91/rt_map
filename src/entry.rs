use crate::{Cell, RefMut};

#[derive(Debug)]
pub struct Entry<'a, K, V> {
    inner: Inner<'a, K, V>,
}

pub type Inner<'a, K, V> = std::collections::hash_map::Entry<'a, K, Cell<V>>;

/// An entry to a resource container.
///
/// This is similar to the Entry API found in the standard library.
///
/// ## Examples
///
/// ```rust
/// use rt_map::RtMap;
///
/// #[derive(Debug)]
/// struct Res(i32);
///
/// let mut rt_map = RtMap::<u32, Res>::default();
///
/// let value = rt_map.entry(0).or_insert(Res(4));
/// println!("{:?}", value.0 * 2);
/// ```
impl<'a, K, V> Entry<'a, K, V> {
    /// Create new entry.
    pub fn new(inner: Inner<'a, K, V>) -> Self {
        Self { inner }
    }

    /// Returns this entry's value, inserts and returns `v` otherwise.
    ///
    /// Please note that you should use `or_insert_with` in case the creation of
    /// the value is expensive.
    pub fn or_insert(self, v: V) -> RefMut<'a, V> {
        self.or_insert_with(move || v)
    }

    /// Returns this entry's value, inserts and returns the return value of `f`
    /// otherwise.
    pub fn or_insert_with<F>(self, f: F) -> RefMut<'a, V>
    where
        F: FnOnce() -> V,
    {
        let inner = self.inner.or_insert_with(move || Cell::new(f()));
        let inner = inner.borrow_mut();

        RefMut::new(inner)
    }
}
