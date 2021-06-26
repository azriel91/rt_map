use std::{cmp::PartialEq, fmt, marker::PhantomData, ops::Deref};

use crate::CellRef;

/// Reference to a value.
pub struct Ref<'a, V>
where
    V: 'a,
{
    pub(crate) inner: CellRef<'a, V>,
    pub(crate) phantom: PhantomData<&'a V>,
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
