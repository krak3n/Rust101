use std::thread;
use std::sync::Arc;

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        return Person { name: name };
    }
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn arc_demo() {
    let name = Arc::new("Chris".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("name = {}", name);

    t.join().unwrap();
}

fn main() {
    arc_demo();
}
