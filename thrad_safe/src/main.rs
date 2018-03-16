use std::thread;
use std::sync::{Arc, Mutex};

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        return Person {
            name: name,
            state: state,
        };
    }
    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("exciting!");
        println!(
            "Hello, my name is {} and I am {}",
            self.name,
            state.as_str()
        );
    }
}

fn arc_demo() {
    let name = Arc::new("Chris".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!(
        "name = {}, state = {}",
        name,
        state.lock().unwrap().as_str()
    );

    t.join().unwrap();
}

fn main() {
    arc_demo();
}
