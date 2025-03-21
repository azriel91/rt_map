# 🗃️ rt_map

[![Crates.io](https://img.shields.io/crates/v/rt_map.svg)](https://crates.io/crates/rt_map)
[![docs.rs](https://img.shields.io/docsrs/rt_map)](https://docs.rs/rt_map)
[![CI](https://github.com/azriel91/rt_map/workflows/CI/badge.svg)](https://github.com/azriel91/rt_map/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/rt_map/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/rt_map)

Runtime managed mutable borrowing from a map.

This library provides a map that allows mutable borrows to different entries at the same time. For a map implementation of this, see [`rt_vec`].

The implementation is extracted and slightly modified from [`shred`].


## Usage

Add the following to `Cargo.toml`

```toml
rt_map = "0.6.0" # or
rt_map = { version = "0.6.0", features = ["unsafe_debug"] }
```

In code:

```rust
use rt_map::RtMap;

struct A(u32);

fn main() {
    let mut rt_map = RtMap::new();

    rt_map.insert('a', A(1));
    rt_map.insert('b', A(2));

    // We can validly have two mutable borrows from the `RtMap` map!
    let mut a = rt_map.borrow_mut(&'a');
    let mut b = rt_map.borrow_mut(&'b');
    a.0 = 2;
    b.0 = 3;

    // We need to explicitly drop the A and B borrows, because they are runtime
    // managed borrows, and rustc doesn't know to drop them before the immutable
    // borrows after this.
    drop(a);
    drop(b);

    // Multiple immutable borrows to the same value are valid.
    let a_0 = rt_map.borrow(&'a');
    let _a_1 = rt_map.borrow(&'a');
    let b = rt_map.borrow(&'b');

    println!("A: {}", a_0.0);
    println!("B: {}", b.0);

    // Trying to mutably borrow a value that is already borrowed (immutably
    // or mutably) returns `Err`.
    let a_try_borrow_mut = rt_map.try_borrow_mut(&'a');
    let exists = if a_try_borrow_mut.is_some() {
        "Ok(..)"
    } else {
        "Err"
    };
    println!("a_try_borrow_mut: {}", exists); // prints "Err"
}
```


### Features

#### `"unsafe_debug"`

Enables the [`"unsafe_debug"`] feature of [`rt_ref`].


## See Also

* [`anymap`]\: Map of any type, without multiple mutable borrows.
* [`resman`]\: Map of any type, with runtime managed borrowing.
* [`shred`]\: Like [`resman`], plus a task dispatcher.


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[`anymap`]: https://github.com/chris-morgan/anymap
[`resman`]: https://github.com/azriel91/resman
[`rt_vec`]: https://crates.io/crates/rt_vec
[`rt_ref`]: https://crates.io/crates/rt_ref
[`shred`]: https://github.com/amethyst/shred
[`"unsafe_debug"`]: https://github.com/azriel91/rt_ref#unsafe_debug
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
