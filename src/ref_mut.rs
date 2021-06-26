use std::{
    cmp::PartialEq,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use crate::{
    cell::Cell, cell_ref::CellRef, cell_ref_mut::CellRefMut, entry::Entry, rt_map::RtMap,
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

#[cfg(test)]
mod tests {
    use std::{
        fmt::{self, Write},
        sync::atomic::AtomicUsize,
    };

    use crate::CellRefMut;

    use super::RefMut;

    #[test]
    fn debug_includes_inner_field() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let mut value = A(1);
        let ref_mut = RefMut::new(CellRefMut {
            flag: &flag,
            value: &mut value,
        });

        let mut debug_string = String::with_capacity(64);
        write!(&mut debug_string, "{:?}", ref_mut)?;
        assert_eq!("RefMut { inner: A(1) }", debug_string.as_str());

        Ok(())
    }

    #[test]
    fn partial_eq_compares_value() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let mut value = A(1);
        let mut value_clone = value.clone();
        let ref_mut = RefMut::new(CellRefMut {
            flag: &flag,
            value: &mut value,
        });

        assert_eq!(
            RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut value_clone,
            }),
            ref_mut
        );
        assert_ne!(
            RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut A(2),
            }),
            ref_mut
        );

        Ok(())
    }

    #[test]
    fn deref_mut_returns_value() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let mut value = A(1);
        let mut ref_mut = RefMut::new(CellRefMut {
            flag: &flag,
            value: &mut value,
        });

        assert_eq!(
            RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut A(1),
            }),
            ref_mut
        );

        ref_mut.0 = 2;

        assert_eq!(
            RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut A(2),
            }),
            ref_mut
        );

        Ok(())
    }

    #[derive(Debug, Clone, PartialEq)]
    struct A(usize);
}
