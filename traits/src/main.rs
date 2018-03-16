trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} can't talk!", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        return Human { name: name };
    }

    fn name(&self) -> &'static str {
        return self.name;
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        return Cat { name: name };
    }

    fn name(&self) -> &'static str {
        return self.name;
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

// Generic trait
trait Summable<T> {
    fn sum(&self) -> T;
}

// add sum to any vector of i32 that returns an i32;
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;
        }
        return result;
    }
}

fn traits() {
    // let h = Human { name: "Chris" };
    // let h = Human::create("Chris");
    let h: Human = Animal::create("Chris");
    h.talk();
    let c = Cat { name: "Izembard" };
    c.talk();

    // Add a trait to a type you do not own using generics
    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum())
}

fn main() {
    traits();
}
