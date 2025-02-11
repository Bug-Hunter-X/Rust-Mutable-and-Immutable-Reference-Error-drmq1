fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modifying x through y
    } 
    let z = &x; // z is an immutable reference to x
    println!("x: {}", x); // This prints 10
    println!("z: {}", *z); //This also prints 10

    //The following code will now compile and will work correctly. 
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    {   
        let z = &x; // z is an immutable reference to x
        println!("x: {}", x); // This prints 10
        println!("z: {}", *z); //This also prints 10
    }
    *y = 15;
    println!("x: {}", x);

} 