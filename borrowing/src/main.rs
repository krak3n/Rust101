fn main() {
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
    };

    let v = vec![1, 2, 3]; // v owns the memory

    // pass a reference (pointer)
    // print_vector borrows the value for a while
    print_vector(&v);
    // v still owns the value
    println!("{:?}", v);

    // a owns the value 40 and is mutable
    let mut a = 40;

    // decare a new scope
    {
        // b borrows a as a mutable reference
        let b = &mut a;

        // increase the value
        *b += 2;
    }
    // b is destroyed and release the borrow on a
    // a now owns it's value again
    println!("{}", a);

    let mut z = vec![3, 2, 1];

    for i in &z {
        println!("i = {}", i);
        z.push(5); // bad so rust says no.
    }
}
