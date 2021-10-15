/// Failures to borrow a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorrowFail {
    /// Value was not found in the map.
    ValueNotFound,
    /// Requested an immutable borrow, but value was already borrowed mutably.
    BorrowConflictImm,
    /// Requested a mutable borrow, but value was already borrowed.
    ///
    /// This variant is returne whether the value was previously borrowed
    /// immutably or mutably.
    BorrowConflictMut,
}
