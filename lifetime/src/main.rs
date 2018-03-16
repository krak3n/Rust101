struct Person {
    name: String,
}

impl Person {
    // what you actually get at compile time.
    // fn get_ref_name<'a>(&'a self) -> &'a String
    fn get_ref_name(&self) -> &String {
        return &self.name;
    }
}

// 'foo sets a lifetime? - z isn't a thing?
struct Company<'z>{
    name: String,
    ceo: &<'z>Person,
}

// &'static str - A lifetime - static = lives as long as the program
fn main() {
    let boss = Person {
        name: String::from("Elon Musk"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };
}
