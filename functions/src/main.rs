fn print_val(x: i32) {
    println!("value = {}", x)
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    return x * y;
    // or x * y
}

fn functions() {
    print_val(33);
    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);
    let a = 3;
    let b = 5;
    let p = product(3, 5);
    println!("p = {}", p);
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        return (dx * dx + dy * dy).sqrt();
    }
}

fn methods() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let line = Line { start: p1, end: p2 };
    println!("length = {}", line.len());
}

fn say_hello() {
    println!("hello world");
}

fn closures() {
    let sh = say_hello;
    sh();
    let plus_on = |x: i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_on(a));

    let two = 2;
    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z
    };
    println!("{} + 2 = {}", a, plus_two(a));

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("{}", f);
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn hof() {
    // imperitive way
    let limit = 500;
    let mut sum = 0;
    // 0.. to infinity and beyond!
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    // functional way
    // #confuddled
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("loop sum2 = {}", sum2);
}

fn main() {
    functions();
    methods();
    closures();
    hof();
}
