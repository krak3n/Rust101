mod pm;

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    println!("Structs");
    println!("-------");
    let p1 = Point { x: 3.0, y: 4.0 };
    println!("point p1 is at ({}, {})", p1.x, p1.y);
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("point p2 is at ({}, {})", p2.x, p2.y);
    let line = Line { start: p1, end: p2 };
    println!(
        "line start ({}, {}) end ({}, {})",
        line.start.x, line.start.y, line.end.x, line.end.y
    );
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple format
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct format
}

fn enums() {
    println!("");
    println!("Enums");
    println!("-----");
    let c: Color = Color::Red;
    let c: Color = Color::RgbColor(255, 0, 0);
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _, // _ 0 - I don't care
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}

union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_iof(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life");
            }
            IntOrFloat { f } => {
                println!("f32 = {}", f);
            }
        }
    }
}

fn unions() {
    println!("");
    println!("Unions");
    println!("------");
    let mut iof = IntOrFloat { i: 123 };
    unsafe {
        iof.i = 42;
    }
    let v = unsafe { iof.i };
    process_iof(iof);
    process_iof(IntOrFloat { f: 1.23 });
}

fn options() {
    println!("");
    println!("Options");
    println!("-------");
    let x = 3.0;
    let y = 2.0;
    // let y = 0.0;

    // Option can contain a Some(z) or None
    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };
    println!("result = {:?}", result);

    match result {
        Some(z) => println!("result of {}/{} = {}", x, y, z),
        None => println!("cannot devide {} by {}", x, y),
    };

    // if let / while let
    if let Some(z) = result {
        println!("z = {}", z);
    }
}

fn arrays() {
    println!("");
    println!("Arrays");
    println!("------");
    // arays are fixed size, they cannot be expanded or contracted
    let mut a: [i32; 5] = [1, 2, 3, 4, 5]; // or let mut a = [1,2,3,4,5];
    println!("len = {}, first = {}", a.len(), a[0]);
    a[0] = 321;
    println!("first = {}", a[0]);
    println!("{:?}", a);
    if a != [1, 2, 3, 4, 5] {
        // can't compare arrays of different lengths
        println!("does not match")
    }
    let b = [1; 10]; // 10 elements all are 1
    for i in 0..b.len() {
        println!("b at {} = {}", i, b[i]);
    }
    println!("b = {} bytes", mem::size_of_val(&b));
    // multi dimentional
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn vectors() {
    println!("");
    println!("Vectors");
    println!("------");
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);
    match a.get(6) {
        // get returns an Option
        Some(x) => println!("a at 6 = {}", x),
        None => println!("no such element"),
    }
    match a.get(2) {
        // get returns an Option
        Some(x) => println!("a at 2 = {}", x),
        None => println!("no such element"),
    }
    for x in &a {
        println!("{}", x);
    }
    a.push(77);
    println!("a = {:?}", a);
    let last_elm = a.pop(); // Option
    match last_elm {
        Some(x) => println!("last element = {}", x),
        None => println!("no last element"),
    }
    println!("a = {:?}", a);
    while let Some(x) = a.pop() {
        println!("x = {}", x);
    }
    println!("a = {:?}", a);
}

fn use_slice(slice: &mut [i32]) {
    println!("first = {}, len = {}", slice[0], slice.len());
    slice[0] = 432
}

fn slices() {
    println!("");
    println!("Slices");
    println!("------");
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    //use_slice(&mut data]);
    println!("data = {:?}", data);
}

fn strings() {
    println!("");
    println!("Slices");
    println!("------");
    // utf-8 characters
    let s = "hello world"; // static &str = string slice - fixed static address
    for c in s.chars() {
        println!("{}", c);
    }
    if let Some(c) = s.chars().nth(0) {
        println!("first char is {}", c);
    }
    // String - heap allocated utf-8 - modifiable
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("letters = {:?}", letters);
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("abc = {}", abc.replace("ello", "good bye"));
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    return (x + y, x * y);
}

fn tuples() {
    println!("");
    println!("Tuples");
    println!("------");
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0}+{1} = {2}, {0}*{1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("{0}+{1} = {2}, {0}*{1} = {3}", x, y, a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2); // tuple of tuples
    println!("combined = {:?}", combined);

    let ((c, d), (e, f)) = combined;
    println!("c = {} d = {} e = {} f = {}", c, d, e, f);
}

struct GenericPoint<T> {
    x: T,
    y: T,
}

struct GenericLine<T> {
    start: GenericPoint<T>,
    end: GenericPoint<T>,
}

fn generics() {
    let a: GenericPoint<u16> = GenericPoint { x: 0, y: 0 };
    let b: GenericPoint<f64> = GenericPoint { x: 1.0, y: 2.3 };
    let c: GenericPoint<u16> = GenericPoint { x: 1, y: 5 };
    let line = GenericLine { start: a, end: c };
}

fn main() {
    structures();
    enums();
    unions();
    options();
    arrays();
    vectors();
    slices();
    strings();
    tuples();
    pm::pattern_matching();
    generics();
}
