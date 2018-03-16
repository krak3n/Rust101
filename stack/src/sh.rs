#![allow(dead_code)]

use std::mem;

struct Point {
    // 16 bytes
    x: f64, // 8 bytes
    y: f64, // 8 bytes
}

// constructs a Point
fn origin() -> Point {
    return Point { x: 0.0, y: 0.0 };
}

pub fn stack_and_heap() {
    println!("stack and heap");

    // stack allocated
    let p1 = origin();
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 16 bytes

    // heap allocated
    let p2 = Box::new(origin()); // p2 is a pointer
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes

    let p3 = *p2; // dereference pointer to get value
    println!("p3.x = {}", p3.x); // 0
}
