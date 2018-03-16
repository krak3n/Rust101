fn main() {
    let v = vec![1, 2, 3]; // v owns the memory

    // this MOVES ownship of the memory of v to v2
    // let v2 = v; // copying a pointer

    // v cannot access v anymore
    // println!("{:?}", v); // won't compile

    let u = 1;
    let u2 = u; // copy for prmitive types since it's cheap

    println!("{:?}", u); // is ok

    // takes ownership and returns it back
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        return x;
    };

    let vv = print_vector(v);
    println!("{}", vv[0]); // is ok
}
