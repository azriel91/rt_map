//! Runtime managed resource borrowing.
//!
//! This library provides a map that can store one of any type, as well as
//! mutable borrows to each type at the same time.
//!
//! **Note:** This implementation is extracted from [`shred`], with the
//! following differences:
//!
//! * Uses [`downcast-rs`] instead of [`mopa`] for downcasting types.
//! * Adds `Debug` and `PartialEq` implementations for borrow types when the
//!   resource type implements those traits.
//! * Returns `None` instead of panicking for `try_borrow*` functions when the
//!   resource is already borrowed.
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! rt_map = "0.1.0"
//! ```
//!
//! In code:
//!
//! ```rust
//! use rt_map::RtMap;
//!
//! struct A(u32);
//! struct B(u32);
//!
//! let mut rt_map = RtMap::new();
//!
//! rt_map.insert(A(1));
//! rt_map.insert(B(2));
//!
//! // We can validly have two mutable borrows from the `RtMap` map!
//! let mut a = rt_map.borrow_mut::<A>();
//! let mut b = rt_map.borrow_mut::<B>();
//! a.0 = 2;
//! b.0 = 3;
//!
//! // We need to explicitly drop the A and B borrows, because they are runtime
//! // managed borrows, and rustc doesn't know to drop them before the immutable
//! // borrows after this.
//! drop(a);
//! drop(b);
//!
//! // Multiple immutable borrows to the same resource are valid.
//! let a_0 = rt_map.borrow::<A>();
//! let _a_1 = rt_map.borrow::<A>();
//! let b = rt_map.borrow::<B>();
//!
//! println!("A: {}", a_0.0);
//! println!("B: {}", b.0);
//!
//! // Trying to mutably borrow a resource that is already borrowed (immutably
//! // or mutably) returns `None`.
//! let a_try_borrow_mut = rt_map.try_borrow_mut::<A>();
//! let exists = if a_try_borrow_mut.is_some() {
//!     "Some(..)"
//! } else {
//!     "None"
//! };
//! println!("a_try_borrow_mut: {}", exists); // prints "None"
//! ```
//!
//! ## See Also
//!
//! * [`anymap`]: Map of any type, without multiple mutable borrows.
//! * [`shred`]: Contains `RtMap` type, plus a task dispatcher.
//!
//! [`anymap`]: https://github.com/chris-morgan/anymap
//! [`downcast-rs`]: https://github.com/marcianx/downcast-rs
//! [`mopa`]: https://github.com/chris-morgan/mopa
//! [`shred`]: https://github.com/amethyst/shred

use std::{
    cmp::PartialEq,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use crate::{
    cell::Cell, cell_ref::CellRef, cell_ref_mut::CellRefMut, entry::Entry, rt_map::RtMap,
};

pub struct Ref<'a, V>
where
    V: 'a,
{
    inner: CellRef<'a, V>,
    phantom: PhantomData<&'a V>,
}

impl<'a, V> Ref<'a, V> {
    pub fn new(inner: CellRef<'a, V>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, V> Deref for Ref<'a, V> {
    type Target = V;

    fn deref(&self) -> &V {
        &*self.inner
    }
}

impl<'a, V> fmt::Debug for Ref<'a, V>
where
    V: fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &V = &*self;
        f.debug_struct("Ref").field("inner", inner).finish()
    }
}

impl<'a, V> PartialEq for Ref<'a, V>
where
    V: PartialEq + 'a,
{
    fn eq(&self, other: &Self) -> bool {
        let r_self: &V = &*self;
        let r_other: &V = &*other;
        r_self == r_other
    }
}

impl<'a, V> Clone for Ref<'a, V> {
    fn clone(&self) -> Self {
        Ref {
            inner: self.inner.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct RefMut<'a, V>
where
    V: 'a,
{
    inner: CellRefMut<'a, V>,
    phantom: PhantomData<&'a V>,
}

impl<'a, V> fmt::Debug for RefMut<'a, V>
where
    V: fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &V = &*self;
        f.debug_struct("RefMut").field("inner", inner).finish()
    }
}

impl<'a, V> PartialEq for RefMut<'a, V>
where
    V: PartialEq + 'a,
{
    fn eq(&self, other: &Self) -> bool {
        let r_self: &V = &*self;
        let r_other: &V = &*other;
        r_self == r_other
    }
}

impl<'a, V> RefMut<'a, V> {
    pub fn new(inner: CellRefMut<'a, V>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, V> Deref for RefMut<'a, V> {
    type Target = V;

    fn deref(&self) -> &V {
        &*self.inner
    }
}

impl<'a, V> DerefMut for RefMut<'a, V> {
    fn deref_mut(&mut self) -> &mut V {
        &mut self.inner
    }
}

mod cell;
mod cell_ref;
mod cell_ref_mut;
mod entry;
mod rt_map;
