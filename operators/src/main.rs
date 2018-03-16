fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = a + 1; // no ++ or --
    println!("a = {}", a);
    a -= 2; // short hand for a = a-2;
    println!("a = {}", a);
    // Remainder
    println!("remainder of = {} / {} = {}", a, 3, (a % 3));

    // bitwise
    let b = 1 | 2; // | OR, & AND, ! NOR - 01 OR 10 == 11 === 3
    println!("1|2 = {}", b);

    // logical
    let c = 1 > 2; // true, < etc
    println!("c = {}", c);
    let d = 1 == 2; // false
    println!("d = {}", d);
    let e = 1 != 2; // true
    println!("e = {}", e);
}

fn main() {
    operators()
}
