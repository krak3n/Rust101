use std::mem;

fn main() {
    // immutable variable
    let a: u8 = 123;
    println!("a = {}", a);

    // explicitly define a variable as muteable
    let mut b: u8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    // Guess the type
    let mut c = 123456789; // int32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    // Creates a varibale the size of the type of the host OS, 32bit, 64bit
    let z: isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, size = {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    // Strings are unicode like go
    let d: char = 'x'; // let d = 'x'; also valid
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // Floats - f64, f32
    let e = 1.23; // double-precision by default (8bytes - float64 (f64))
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // Bools
    let f = false;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    let g = 4 > 0; // true
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
