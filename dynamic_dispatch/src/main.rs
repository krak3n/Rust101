trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        return format!("i32: {}", *self);
    }
}

impl Printable for String {
    fn format(&self) -> String {
        return format!("string: {}", *self);
    }
}

fn print_it(z: &Printable) {
    // which format to run happens at runtime
    // and is therefore more expensive vs static_dispatch
    println!("{}", z.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();
    print_it(&a);
    print_it(&b);
}
