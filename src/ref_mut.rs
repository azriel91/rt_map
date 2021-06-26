use std::{
    cmp::PartialEq,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use crate::{
    cell::Cell, cell_ref::CellRef, cell_ref_mut::CellRefMut, entry::Entry, r#ref::Ref,
    rt_map::RtMap,
};

/// Mutable references to a value.
pub struct RefMut<'a, V>
where
    V: 'a,
{
    pub(crate) inner: CellRefMut<'a, V>,
    pub(crate) phantom: PhantomData<&'a V>,
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
