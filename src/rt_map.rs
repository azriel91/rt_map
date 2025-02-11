use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use rt_ref::{BorrowFail, Cell, Ref, RefMut};

use crate::Entry;

/// Map from `TypeId` to type.
#[derive(Debug)]
pub struct RtMap<K, V>(HashMap<K, Cell<V>>);

impl<K, V> Default for RtMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

macro_rules! borrow_panic {
    ($key:ident) => {
        panic!(
            "Expected to borrow `{key:?}`, but it does not exist.",
            key = $key
        )
    };
}

/// A [`HashMap`] that allows multiple mutable borrows to different entries.
///
/// The [`borrow`] and [`borrow_mut`] methods take `&self`, allowing multiple
/// mutable borrows of different entries at the same time. This is achieved via
/// interior mutability. In case you violate the borrowing rules of Rust
/// (multiple reads xor one write), you will get a panic.
///
/// For non-packing versions of these methods, use [`try_borrow`] and
/// [`try_borrow_mut`].
///
/// [`borrow`]: Self::borrow
/// [`borrow_mut`]: Self::borrow_mut
/// [`try_borrow`]: Self::try_borrow
/// [`try_borrow_mut`]: Self::try_borrow_mut
impl<K, V> RtMap<K, V>
where
    K: Hash + Eq,
{
    /// Creates an empty `RtMap`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtMap;
    /// let mut map = RtMap::<u32, String>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `RtMap` with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtMap;
    /// let map: RtMap<&str, i32> = RtMap::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `RtMap<K, V>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtMap;
    /// let map: RtMap<i32, i32> = RtMap::with_capacity(100);
    /// assert!(map.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> HashMap<K, Cell<V>> {
        self.0
    }

    /// Gets the given key’s corresponding entry in the map for in-place
    /// manipulation.
    pub fn entry(&mut self, k: K) -> Entry<'_, K, V> {
        Entry::new(self.0.entry(k))
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtMap;
    ///
    /// let mut map = RtMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(*map.borrow(&37), "c");
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k, Cell::new(v)).map(Cell::into_inner)
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtMap;
    ///
    /// let mut a = RtMap::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a");
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map’s key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtMap;
    ///
    /// let mut map = RtMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.0.remove(k).map(Cell::into_inner)
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and
    /// [`Eq`] on the borrowed form must match those for the key type.
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.0.contains_key(k)
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and
    /// [`Eq`] on the borrowed form must match those for the key type.
    ///
    /// See [`try_borrow`] for a non-panicking version of this function.
    ///
    /// # Panics
    ///
    /// * Panics if the resource doesn't exist.
    /// * Panics if the resource is being accessed mutably.
    ///
    /// [`try_borrow`]: Self::try_borrow
    pub fn borrow<Q>(&self, k: &Q) -> Ref<V>
    where
        Q: ?Sized + Hash + Eq + fmt::Debug,
        K: Borrow<Q>,
    {
        self.0
            .get(k)
            .map(|cell| Ref::new(cell.borrow()))
            .unwrap_or_else(|| borrow_panic!(k))
    }

    /// Returns a reference to the value if it exists and is not mutably
    /// borrowed, `None` otherwise.
    pub fn try_borrow<Q>(&self, k: &Q) -> Result<Ref<V>, BorrowFail>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.0
            .get(k)
            .ok_or(BorrowFail::ValueNotFound)
            .and_then(|cell| cell.try_borrow().map(Ref::new))
    }

    /// Returns a reference to the value if it exists and is not borrowed,
    /// `None` otherwise.
    ///
    /// # Panics
    ///
    /// * Panics if the resource doesn't exist.
    /// * Panics if the resource is already accessed.
    pub fn borrow_mut<Q>(&self, k: &Q) -> RefMut<V>
    where
        Q: ?Sized + Hash + Eq + fmt::Debug,
        K: Borrow<Q>,
    {
        self.0
            .get(k)
            .map(|cell| RefMut::new(cell.borrow_mut()))
            .unwrap_or_else(|| borrow_panic!(k))
    }

    /// Returns a mutable reference to `R` if it exists, `None` otherwise.
    pub fn try_borrow_mut<Q>(&self, k: &Q) -> Result<RefMut<V>, BorrowFail>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.0
            .get(k)
            .ok_or(BorrowFail::ValueNotFound)
            .and_then(|r_cell| r_cell.try_borrow_mut().map(RefMut::new))
    }

    /// Retrieves a resource without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.get_resource_mut(k)
    }

    /// Retrieves a resource without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_resource_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.0.get_mut(k).map(Cell::get_mut)
    }

    /// Get raw access to the underlying cell.
    pub fn get_raw<Q>(&self, k: &Q) -> Option<&Cell<V>>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.0.get(k)
    }
}

impl<K, V> Deref for RtMap<K, V> {
    type Target = HashMap<K, Cell<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for RtMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use rt_ref::BorrowFail;

    use super::RtMap;

    #[derive(Debug, Default, PartialEq)]
    struct Res;

    #[derive(Debug, Default, PartialEq)]
    struct Value(u32);

    #[test]
    fn insert() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        assert!(rt_map.contains_key(&'a'));
        assert!(!rt_map.contains_key(&'b'));
    }

    #[test]
    fn with_capacity_reserves_enough_capacity() {
        let map: RtMap<i32, i32> = RtMap::with_capacity(100);
        assert!(map.capacity() >= 100);
    }

    #[test]
    fn into_inner() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);
        let inner_map = rt_map.into_inner();

        assert!(inner_map.contains_key(&'a'));
    }

    #[test]
    fn deref_and_deref_mut() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', 0);
        rt_map.insert('b', 1);

        rt_map.iter_mut().for_each(|(_k, v)| *v.borrow_mut() += 1);

        let a = rt_map.remove(&'a');
        assert_eq!(Some(1), a);

        let b = rt_map.iter().next();
        assert_eq!(Some(2), b.map(|(_k, v)| *v.borrow()));
    }

    #[test]
    fn is_empty_returns_true_when_map_does_not_contain_items() {
        let rt_map = RtMap::<char, u32>::new();

        assert!(rt_map.is_empty());
    }

    #[test]
    fn is_empty_returns_false_when_map_contains_items() {
        let mut rt_map = RtMap::new();

        rt_map.insert('a', 0);

        assert!(!rt_map.is_empty());
    }

    #[test]
    fn entry_insert_value() {
        let mut rt_map = RtMap::new();

        let ref_mut = rt_map.entry('a').or_insert(1);

        assert_eq!(1, *ref_mut);
        drop(ref_mut);

        let ref_mut = rt_map.entry('a').or_insert(2);

        assert_eq!(1, *ref_mut);
        drop(ref_mut);

        rt_map.remove(&'a');

        let ref_mut = rt_map.entry('a').or_insert_with(|| 3);

        assert_eq!(3, *ref_mut);
        drop(ref_mut);
    }

    #[test]
    fn get_mut_returns_mutable_reference_to_value() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Value(1));

        let value = rt_map.get_mut(&'a');

        assert!(value.is_some());

        if let Some(value) = value {
            *value = Value(2);
        }

        let value = rt_map.get_mut(&'a').map(|value| value.0);

        assert_eq!(Some(2), value);
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed")]
    fn read_write_fails() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        let _read = rt_map.borrow(&'a');
        let _write = rt_map.borrow_mut(&'a');
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed mutably")]
    fn write_read_fails() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        let _write = rt_map.borrow_mut(&'a');
        let _read = rt_map.borrow(&'a');
    }

    #[test]
    fn remove_insert() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        assert!(rt_map.contains_key(&'a'));

        rt_map.remove(&'a').unwrap();

        assert!(!rt_map.contains_key(&'a'));

        rt_map.insert('a', Res);

        assert!(rt_map.contains_key(&'a'));
    }

    #[test]
    #[should_panic(expected = "Expected to borrow `'a'`, but it does not exist.")]
    fn borrow_before_insert_panics() {
        let rt_map = RtMap::<char, i32>::new();

        rt_map.borrow(&'a');
    }

    #[test]
    #[should_panic(expected = "Expected to borrow `'a'`, but it does not exist.")]
    fn borrow_mut_before_insert_panics() {
        let rt_map = RtMap::<char, i32>::new();

        rt_map.borrow_mut(&'a');
    }

    #[test]
    fn borrow_mut_try_borrow_returns_borrow_conflict_imm() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        let _res = rt_map.borrow_mut(&'a');

        assert_eq!(Err(BorrowFail::BorrowConflictImm), rt_map.try_borrow(&'a'));
    }

    #[test]
    fn borrow_try_borrow_mut_returns_borrow_conflict_mut() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        let _res = rt_map.borrow(&'a');

        assert_eq!(
            Err(BorrowFail::BorrowConflictMut),
            rt_map.try_borrow_mut(&'a')
        );
    }

    #[test]
    fn borrow_mut_borrow_mut_returns_borrow_conflict_mut() {
        let mut rt_map = RtMap::new();
        rt_map.insert('a', Res);

        let _res = rt_map.borrow_mut(&'a');

        assert_eq!(
            Err(BorrowFail::BorrowConflictMut),
            rt_map.try_borrow_mut(&'a')
        );
    }

    #[test]
    fn try_borrow_before_insert_returns_value_not_found() {
        let rt_map = RtMap::<char, Res>::new();

        assert_eq!(Err(BorrowFail::ValueNotFound), rt_map.try_borrow(&'a'));
    }

    #[test]
    fn try_borrow_mut_before_insert_returns_value_not_found() {
        let rt_map = RtMap::<char, Res>::new();

        assert_eq!(Err(BorrowFail::ValueNotFound), rt_map.try_borrow_mut(&'a'));
    }

    #[test]
    #[should_panic(expected = "Expected to borrow `'a'`, but it does not exist.")]
    fn borrow_before_insert_panics_value_not_found() {
        let rt_map = RtMap::<char, Res>::new();

        rt_map.borrow(&'a');
    }

    #[test]
    #[should_panic(expected = "Expected to borrow `'a'`, but it does not exist.")]
    fn borrow_mut_before_insert_panics_value_not_found() {
        let rt_map = RtMap::<char, Res>::new();

        rt_map.borrow_mut(&'a');
    }
}
