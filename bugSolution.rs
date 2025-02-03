fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // Output: x = 6
    
    // The immutable borrow is removed, so this now works fine
    println!("x = {}", x); // Output: x = 6
} 