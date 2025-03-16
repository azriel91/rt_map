# Changelog

## unreleased

* Update crate to rust edition 2024.


## 0.5.4 (2024-10-19)

* Add `RtMap::into_inner`.


## 0.5.3 (2024-08-31)

* Update dependency versions.


## 0.5.2 (2022-07-15)

* Remove `RtVec` -- split to the `rt_vec` crate. ([#2])
* Add `"unsafe_debug"` passthrough feature. [#3]

[#2]: https://github.com/azriel91/rt_map/pull/2
[#3]: https://github.com/azriel91/rt_map/pull/3
[`rt_vec`]: https://crates.io/crates/rt_vec


## 0.5.1 (2022-06-25)

* Add `RtVec` gated behind `"rt_vec"` feature. ([#1])

[#1]: https://github.com/azriel91/rt_map/pull/1


## 0.5.0 (2021-10-16)

* Return `BorrowFail` indicating the reason when failing to borrow a value.


## 0.4.0 (2021-08-08)

* Implement `Deref` and `DerefMut` for `RtMap`.


## 0.3.0 (2021-08-01)

* Implement `Debug` for `RtMap`.


## 0.2.0 (2021-08-01)

* Add `RtMap::capacity` and `RtMap::with_capacity`.


## 0.1.0 (2021-06-26)

* Add `RtMap`.
