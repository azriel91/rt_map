//! Runtime managed mutable borrowing from a map or vec.
//!
//! This library provides a map and vec that allows mutable borrows to different
//! entries at the same time.
//!
//! This implementation is extracted and slightly modified from [`shred`].
//!
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! rt_map = "0.5.1"
//! rt_map = { version = "0.5.1", features = ["rt_vec"] } # to enable `RtVec`
//! ```
//!
//!
//! ### [`RtMap`]
//!
//! ```rust
//! use rt_map::RtMap;
//!
//! struct A(u32);
//!
//! let mut rt_map = RtMap::new();
//!
//! rt_map.insert('a', A(1));
//! rt_map.insert('b', A(2));
//!
//! // We can validly have two mutable borrows from the `RtMap` map!
//! let mut a = rt_map.borrow_mut(&'a');
//! let mut b = rt_map.borrow_mut(&'b');
//! a.0 = 2;
//! b.0 = 3;
//!
//! // We need to explicitly drop the A and B borrows, because they are runtime
//! // managed borrows, and rustc doesn't know to drop them before the immutable
//! // borrows after this.
//! drop(a);
//! drop(b);
//!
//! // Multiple immutable borrows to the same value are valid.
//! let a_0 = rt_map.borrow(&'a');
//! let _a_1 = rt_map.borrow(&'a');
//! let b = rt_map.borrow(&'b');
//!
//! println!("A: {}", a_0.0);
//! println!("B: {}", b.0);
//!
//! // Trying to mutably borrow a value that is already borrowed (immutably
//! // or mutably) returns `Err`.
//! let a_try_borrow_mut = rt_map.try_borrow_mut(&'a');
//! let exists = if a_try_borrow_mut.is_ok() {
//!     "Ok(..)"
//! } else {
//!     "Err"
//! };
//! println!("a_try_borrow_mut: {}", exists); // prints "Err"
//! ```
//!
//!
//! ### [`RtVec`]
//!
//! ```rust
//! use rt_map::RtVec;
//!
//! struct A(u32);
//!
//! let mut rt_vec = RtVec::new();
//!
//! rt_vec.push(A(1));
//! rt_vec.push(A(2));
//!
//! // We can validly have two mutable borrows from the `RtVec` map!
//! let mut a = rt_vec.borrow_mut(0);
//! let mut b = rt_vec.borrow_mut(1);
//! a.0 = 2;
//! b.0 = 3;
//!
//! // We need to explicitly drop the A and B borrows, because they are runtime
//! // managed borrows, and rustc doesn't know to drop them before the immutable
//! // borrows after this.
//! drop(a);
//! drop(b);
//!
//! // Multiple immutable borrows to the same value are valid.
//! let a_0 = rt_vec.borrow(0);
//! let _a_1 = rt_vec.borrow(0);
//! let b = rt_vec.borrow(1);
//!
//! println!("A: {}", a_0.0);
//! println!("B: {}", b.0);
//!
//! // Trying to mutably borrow a value that is already borrowed (immutably
//! // or mutably) returns `Err`.
//! let a_try_borrow_mut = rt_vec.try_borrow_mut(0);
//! let exists = if a_try_borrow_mut.is_ok() {
//!     "Ok(..)"
//! } else {
//!     "Err"
//! };
//! println!("a_try_borrow_mut: {}", exists); // prints "Err"
//! ```
//!
//!
//! ## See Also
//!
//! * [`anymap`]\: Map of any type, without multiple mutable borrows.
//! * [`resman`]\: Map of any type, with runtime managed borrowing.
//! * [`shred`]\: Contains `RtMap` type, plus a task dispatcher.
//!
//! [`anymap`]: https://github.com/chris-morgan/anymap
//! [`resman`]: https://github.com/azriel91/resman
//! [`shred`]: https://github.com/amethyst/shred

pub use crate::{
    borrow_fail::BorrowFail, cell::Cell, cell_ref::CellRef, cell_ref_mut::CellRefMut, entry::Entry,
    r#ref::Ref, ref_mut::RefMut, rt_map::RtMap,
};

mod borrow_fail;
mod cell;
mod cell_ref;
mod cell_ref_mut;
mod entry;
mod r#ref;
mod ref_mut;
mod rt_map;

#[cfg(feature = "rt_vec")]
pub use crate::rt_vec::RtVec;

#[cfg(feature = "rt_vec")]
mod rt_vec;
