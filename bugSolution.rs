fn main() {
    let mut x = 5;
    { // The inner block limits the scope of y
        let y = &mut x;
        *y = 10; // Modify x through y
        println!("x = {}", x); // Prints 10
    }
    //Outside of this block, y is no longer valid and can't be used again.

    x = 15;  //Modify x directly
    println!("x = {}", x); // Prints 15
} 