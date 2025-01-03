fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 6;
    println!("x = {}", x); //Prints 6

    //To have multiple mutable references, one could use cloning
    let mut z = x.clone(); //z is now independent
    z = 7; //This is allowed because z is a completely separate value
    println!("z = {}", z); //Prints 7
    println!("x = {}", x); //Still prints 6

    //Another solution involves using RefCell to allow interior mutability
    use std::cell::RefCell;

    let x = RefCell::new(5);
    let mut y = x.borrow_mut();
    *y = 6;
    let mut z = x.borrow_mut();
    *z = 7; //This is now possible because RefCell handles interior mutability
    println!("x = {}", *x.borrow()); //Prints 7
}