use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        return Person { name: name };
    }
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("Chris".to_string());
    println!(
        "Name = {}, has {} string pointers",
        name,
        Rc::strong_count(&name)
    );
    {
        let person = Person::new(name.clone());
        println!(
            "Name = {}, has {} string pointers",
            name,
            Rc::strong_count(&name)
        );
        person.greet();
    }
    println!(
        "Name = {}, has {} string pointers",
        name,
        Rc::strong_count(&name)
    );
    println!("{}", name);
}

fn main() {
    rc_demo();
}
