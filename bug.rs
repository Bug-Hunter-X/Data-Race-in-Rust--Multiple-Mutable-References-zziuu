fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x
    *y = 6; // Modifies x through y
    *z = 7; // This will lead to a data race, which is undefined behavior in Rust
    println!("x = {}", x);
}