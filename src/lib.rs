//! Runtime managed mutable borrowing from a map.
//!
//! This library provides a map that allows mutable borrows to different entries
//! at the same time. For a map implementation of this, see [`rt_vec`].
//!
//! The implementation is extracted and slightly modified from [`shred`].
//!
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! rt_map = "0.5.1" # or
//! rt_map = { version = "0.5.1", features = ["unsafe_debug"] }
//! ```
//!
//! In code:
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
//! ### Features
//!
//! #### `"unsafe_debug"`
//!
//! Enables the [`"unsafe_debug"`] feature of [`rt_ref`].
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
//! [`rt_vec`]: https://crates.io/crates/rt_vec
//! [`shred`]: https://github.com/amethyst/shred
//! [`"unsafe_debug"`]: https://github.com/azriel91/rt_ref#unsafe_debug

// Re-exports
pub use rt_ref::{BorrowFail, Cell, CellRef, CellRefMut, Ref, RefMut};

pub use crate::{entry::Entry, rt_map::RtMap};

mod entry;
mod rt_map;
