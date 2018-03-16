fn scoped() {
    let a = 123;
    println!("a = {}", a);
    {
        let b = 456;
        println!("b = {}", b);

        let a = 789; // shadows a
        println!("a = {}", a);
    }
    // println!("b = {}", b); - can't printb
    println!("a = {}", a);
    let a = 321; // can redecare
    println!("a = {}", a);
}

fn main() {
    scoped()
}
