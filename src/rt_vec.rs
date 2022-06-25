use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{BorrowFail, Cell, Ref, RefMut};

/// Map from `TypeId` to type.
#[derive(Debug)]
pub struct RtVec<V>(Vec<Cell<V>>);

impl<V> Default for RtVec<V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

macro_rules! borrow_panic {
    ($index:ident) => {
        panic!(
            "Expected to borrow index `{index}`, but it does not exist.",
            index = $index
        )
    };
}

/// A [`Vec`] that allows multiple mutable borrows to different entries.
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
impl<V> RtVec<V> {
    /// Creates an empty `RtVec`.
    ///
    /// The `RtVec` is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtVec;
    ///
    /// let mut rt_vec = RtVec::<u32>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `RtVec` with the specified capacity.
    ///
    /// The `RtVec` will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the vec will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtVec;
    ///
    /// let rt_vec: RtVec<i32> = RtVec::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Returns the number of elements the vec can hold without reallocating.
    ///
    /// This number is a lower bound; the `RtVec<V>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rt_map::RtVec;
    /// let rt_vec: RtVec<i32> = RtVec::with_capacity(100);
    /// assert!(rt_vec.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rt_map::RtVec;
    /// #
    /// let mut rt_vec = RtVec::new();
    ///
    /// rt_vec.push('a');
    /// rt_vec.push('b');
    /// assert_eq!(*rt_vec.borrow(0), 'a');
    /// assert_eq!(*rt_vec.borrow(1), 'b');
    /// ```
    pub fn push(&mut self, v: V) {
        self.0.push(Cell::new(v));
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rt_map::RtVec;
    /// #
    /// let mut rt_vec = RtVec::new();
    /// rt_vec.push('a');
    /// rt_vec.insert(0, 'b');
    ///
    /// assert_eq!(*rt_vec.borrow(0), 'b');
    /// assert_eq!(*rt_vec.borrow(1), 'a');
    /// ```
    pub fn insert(&mut self, index: usize, v: V) {
        self.0.insert(index, Cell::new(v));
    }

    /// Returns `true` if the vec contains no elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rt_map::RtVec;
    /// #
    /// let mut rt_vec = RtVec::new();
    /// assert!(rt_vec.is_empty());
    ///
    /// rt_vec.push('a');
    /// assert!(!rt_vec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O(n)*. If you don’t need the order of
    /// elements to be preserved, use [`swap_remove`] instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rt_map::RtVec;
    /// #
    /// let mut rt_vec = RtVec::new();
    /// rt_vec.push('a');
    /// assert_eq!(rt_vec.remove(0), 'a');
    /// ```
    ///
    /// [`swap_remove`]: Self::swap_remove
    pub fn remove(&mut self, index: usize) -> V {
        let cell = self.0.remove(index);
        Cell::into_inner(cell)
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is *O*(1). If you need to preserve
    /// the element order, use [`remove`] instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rt_map::RtVec;
    /// #
    /// let mut v = vec!["foo", "bar", "baz", "qux"];
    ///
    /// assert_eq!(v.swap_remove(1), "bar");
    /// assert_eq!(v, ["foo", "qux", "baz"]);
    ///
    /// assert_eq!(v.swap_remove(0), "foo");
    /// assert_eq!(v, ["baz", "qux"]);
    /// ```
    ///
    /// [`remove`]: Self::remove
    pub fn swap_remove(&mut self, index: usize) -> V {
        let cell = self.0.swap_remove(index);
        Cell::into_inner(cell)
    }

    /// Returns a reference to the value corresponding to the index.
    ///
    /// See [`try_borrow`] for a non-panicking version of this function.
    ///
    /// # Panics
    ///
    /// * Panics if the value is already borrowed mutably.
    ///
    /// [`try_borrow`]: Self::try_borrow
    pub fn borrow(&self, index: usize) -> Ref<V> {
        self.0
            .get(index)
            .map(|cell| Ref {
                inner: cell.borrow(),
                phantom: PhantomData,
            })
            .unwrap_or_else(|| borrow_panic!(index))
    }

    /// Returns a reference to the value if it exists and is not mutably
    /// borrowed.
    ///
    /// * Returns `BorrowFail::ValueNotFound` if `index` is out of bounds.
    /// * Returns `BorrowFail::BorrowConflictImm` if the value is already
    ///   borrowed mutably.
    pub fn try_borrow(&self, index: usize) -> Result<Ref<V>, BorrowFail> {
        self.0
            .get(index)
            .ok_or(BorrowFail::ValueNotFound)
            .and_then(|cell| {
                cell.try_borrow().map(|cell_ref| Ref {
                    inner: cell_ref,
                    phantom: PhantomData,
                })
            })
    }

    /// Returns a reference to the value if it exists and is not borrowed.
    ///
    /// See [`try_borrow_mut`] for a non-panicking version of this function.
    ///
    /// # Panics
    ///
    /// Panics if the value is already borrowed either immutably or mutably.
    ///
    /// [`try_borrow_mut`]: Self::try_borrow_mut
    pub fn borrow_mut(&self, index: usize) -> RefMut<V> {
        self.0
            .get(index)
            .map(|cell| RefMut {
                inner: cell.borrow_mut(),
                phantom: PhantomData,
            })
            .unwrap_or_else(|| borrow_panic!(index))
    }

    /// Returns a mutable reference to the value if it is successfully borrowed
    /// mutably.
    ///
    /// * Returns `BorrowFail::ValueNotFound` if `index` is out of bounds.
    /// * Returns `BorrowFail::BorrowConflictMut` if the value is already
    ///   borrowed.
    pub fn try_borrow_mut(&self, index: usize) -> Result<RefMut<V>, BorrowFail> {
        self.0
            .get(index)
            .ok_or(BorrowFail::ValueNotFound)
            .and_then(|r_cell| {
                r_cell.try_borrow_mut().map(|cell_ref_mut| RefMut {
                    inner: cell_ref_mut,
                    phantom: PhantomData,
                })
            })
    }

    /// Returns a reference to the value corresponding to the index if it is not
    /// borrowed.
    ///
    /// Returns `None` if `index` is out of bounds.
    ///
    /// See [`borrow`] for a version of this that returns a `Result`
    ///
    /// # Panics
    ///
    /// Panics if the value is already borrowed mutably.
    pub fn get(&self, index: usize) -> Option<Ref<V>> {
        self.0.get(index).map(|cell| Ref {
            inner: cell.borrow(),
            phantom: PhantomData,
        })
    }

    /// Retrieves a value without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut V> {
        self.get_value_mut(index)
    }

    /// Retrieves a value without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_value_mut(&mut self, index: usize) -> Option<&mut V> {
        self.0.get_mut(index).map(Cell::get_mut)
    }

    /// Get raw access to the underlying cell.
    pub fn get_raw(&self, index: usize) -> Option<&Cell<V>> {
        self.0.get(index)
    }
}

impl<V> Deref for RtVec<V> {
    type Target = Vec<Cell<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for RtVec<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::RtVec;
    use crate::BorrowFail;

    #[derive(Debug, Default, PartialEq)]
    struct Res;

    #[derive(Debug, Default, PartialEq)]
    struct Value(u32);

    #[test]
    fn insert() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        assert_eq!(Res, *rt_vec.borrow(0));
    }

    #[test]
    fn with_capacity_reserves_enough_capacity() {
        let rt_vec: RtVec<i32> = RtVec::with_capacity(100);
        assert!(rt_vec.capacity() >= 100);
    }

    #[test]
    fn deref_and_deref_mut() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, 0);
        rt_vec.insert(1, 1);

        rt_vec.iter_mut().for_each(|v| *v.borrow_mut() += 1);

        let a = rt_vec.remove(0);
        assert_eq!(1, a);

        let b = rt_vec.iter().next();
        assert_eq!(Some(2), b.map(|v| *v.borrow()));
    }

    #[test]
    fn is_empty_returns_true_when_map_does_not_contain_items() {
        let rt_vec = RtVec::<u32>::new();

        assert!(rt_vec.is_empty());
    }

    #[test]
    fn is_empty_returns_false_when_map_contains_items() {
        let mut rt_vec = RtVec::new();

        rt_vec.insert(0, 0);

        assert!(!rt_vec.is_empty());
    }

    #[test]
    fn get_mut_returns_mutable_reference_to_value() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Value(1));

        let value = rt_vec.get_mut(0);

        assert!(value.is_some());

        if let Some(value) = value {
            *value = Value(2);
        }

        let value = rt_vec.get_mut(0).map(|value| value.0);

        assert_eq!(Some(2), value);
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed")]
    fn read_write_fails() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        let _read = rt_vec.borrow(0);
        let _write = rt_vec.borrow_mut(0);
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed mutably")]
    fn write_read_fails() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        let _write = rt_vec.borrow_mut(0);
        let _read = rt_vec.borrow(0);
    }

    #[test]
    fn remove_insert() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        assert!(rt_vec.get(0).is_some());

        rt_vec.remove(0);

        assert!(rt_vec.get(0).is_none());

        rt_vec.insert(0, Res);

        assert!(rt_vec.get(0).is_some());
    }

    #[test]
    #[should_panic(expected = "Expected to borrow index `0`, but it does not exist.")]
    fn borrow_before_insert_panics() {
        let rt_vec = RtVec::<i32>::new();

        rt_vec.borrow(0);
    }

    #[test]
    #[should_panic(expected = "Expected to borrow index `0`, but it does not exist.")]
    fn borrow_mut_before_insert_panics() {
        let rt_vec = RtVec::<i32>::new();

        rt_vec.borrow_mut(0);
    }

    #[test]
    fn borrow_mut_try_borrow_returns_borrow_conflict_imm() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        let _res = rt_vec.borrow_mut(0);

        assert_eq!(Err(BorrowFail::BorrowConflictImm), rt_vec.try_borrow(0));
    }

    #[test]
    fn borrow_try_borrow_mut_returns_borrow_conflict_mut() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        let _res = rt_vec.borrow(0);

        assert_eq!(Err(BorrowFail::BorrowConflictMut), rt_vec.try_borrow_mut(0));
    }

    #[test]
    fn borrow_mut_borrow_mut_returns_borrow_conflict_mut() {
        let mut rt_vec = RtVec::new();
        rt_vec.insert(0, Res);

        let _res = rt_vec.borrow_mut(0);

        assert_eq!(Err(BorrowFail::BorrowConflictMut), rt_vec.try_borrow_mut(0));
    }

    #[test]
    fn try_borrow_before_insert_returns_value_not_found() {
        let rt_vec = RtVec::<Res>::new();

        assert_eq!(Err(BorrowFail::ValueNotFound), rt_vec.try_borrow(0));
    }

    #[test]
    fn try_borrow_mut_before_insert_returns_value_not_found() {
        let rt_vec = RtVec::<Res>::new();

        assert_eq!(Err(BorrowFail::ValueNotFound), rt_vec.try_borrow_mut(0));
    }

    #[test]
    #[should_panic(expected = "Expected to borrow index `0`, but it does not exist.")]
    fn borrow_before_insert_panics_value_not_found() {
        let rt_vec = RtVec::<Res>::new();

        rt_vec.borrow(0);
    }

    #[test]
    #[should_panic(expected = "Expected to borrow index `0`, but it does not exist.")]
    fn borrow_mut_before_insert_panics_value_not_found() {
        let rt_vec = RtVec::<Res>::new();

        rt_vec.borrow_mut(0);
    }
}
