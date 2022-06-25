use rt_map::RtVec;

struct A(u32);

fn main() {
    let mut rt_vec = RtVec::new();

    rt_vec.push(A(1));
    rt_vec.push(A(2));

    // We can validly have two mutable borrows from the `RtVec` map!
    let mut a = rt_vec.borrow_mut(0);
    let mut b = rt_vec.borrow_mut(1);
    a.0 = 2;
    b.0 = 3;

    // We need to explicitly drop the A and B borrows, because they are runtime
    // managed borrows, and rustc doesn't know to drop them before the immutable
    // borrows after this.
    drop(a);
    drop(b);

    // Multiple immutable borrows to the same value are valid.
    let a_0 = rt_vec.borrow(0);
    let _a_1 = rt_vec.borrow(0);
    let b = rt_vec.borrow(1);

    println!("A: {}", a_0.0);
    println!("B: {}", b.0);

    // Trying to mutably borrow a value that is already borrowed (immutably
    // or mutably) returns `Err`.
    let a_try_borrow_mut = rt_vec.try_borrow_mut(0);
    let exists = if a_try_borrow_mut.is_ok() {
        "Ok(..)"
    } else {
        "Err"
    };

    println!("a_try_borrow_mut: {}", exists); // prints "Err"
}
