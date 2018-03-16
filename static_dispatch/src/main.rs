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

// Monomorphisation
// At compile time what you actualy get is:
// fn print_it(z: String) {
//     ...
// }
//// fn print_it(z: i32) {
//     ...
// }
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();
    print_it(a);
    print_it(b);
}
