fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 10; // This is fine
    let z = y; // z is also a mutable reference to x
    *z = 15; // This is also fine
    let w = y; // w is a mutable reference to x
    *w = 20;//This is also fine
    println!("x = {}", x); // x = 20
    *y = 25; // This will cause a compiler error
    println!("x = {}", x); 
}