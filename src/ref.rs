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

#[cfg(test)]
mod tests {
    use std::{
        fmt::{self, Write},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use crate::CellRef;

    use super::Ref;

    #[test]
    fn debug_includes_inner_field() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let value = A(1);
        let r#ref = Ref::new(CellRef {
            flag: &flag,
            value: &value,
        });

        let mut debug_string = String::with_capacity(64);
        write!(&mut debug_string, "{:?}", r#ref)?;
        assert_eq!("Ref { inner: A(1) }", debug_string.as_str());

        Ok(())
    }

    #[test]
    fn partial_eq_compares_value() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let value = A(1);
        let r#ref = Ref::new(CellRef {
            flag: &flag,
            value: &value,
        });

        assert_eq!(
            Ref::new(CellRef {
                flag: &flag,
                value: &value,
            }),
            r#ref
        );
        assert_ne!(
            Ref::new(CellRef {
                flag: &flag,
                value: &A(2),
            }),
            r#ref
        );

        Ok(())
    }

    #[test]
    fn clone_increments_cell_ref_count() -> fmt::Result {
        let flag = AtomicUsize::new(1);
        let value = A(1);
        let ref_0 = Ref::new(CellRef {
            flag: &flag,
            value: &value,
        });

        assert_eq!(1, ref_0.inner.flag.load(Ordering::SeqCst));

        let ref_1 = ref_0.clone();

        assert_eq!(2, ref_0.inner.flag.load(Ordering::SeqCst));
        assert_eq!(2, ref_1.inner.flag.load(Ordering::SeqCst));

        Ok(())
    }

    #[derive(Debug, Clone, PartialEq)]
    struct A(usize);
}
