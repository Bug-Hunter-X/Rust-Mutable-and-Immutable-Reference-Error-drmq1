fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10; // Modifying x through y
    let z = &x; // z is an immutable reference to x
    println!("x: {}", x); // This prints 10
    println!("z: {}", *z); //This also prints 10

    //  The following code will compile but will cause a runtime error:
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x
    *y = 10; // Modifying x through y
    println!("x: {}", x); // This prints 10
    println!("z: {}", *z); //This also prints 10

    // The following will also fail compilation because it violates the rules about mutable references
    //   let mut x = 5;
    //   let y = &mut x; // y is a mutable reference to x
    //   let z = &mut x; // z is a mutable reference to x
    //   *y = 10;
    //   *z = 15;
    //   println!("x: {}", x); // This will print either 10 or 15, but the order of operations is undefined

}