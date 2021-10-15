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
    let exists = if a_try_borrow_mut.is_ok() {
        "Ok(..)"
    } else {
        "Err"
    };
    println!("a_try_borrow_mut: {}", exists); // prints "Err"
}
