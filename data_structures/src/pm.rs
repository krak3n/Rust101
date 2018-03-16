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

pub fn pattern_matching() {
    println!("");
    println!("Pattern Matching");
    println!("------");
    for x in 0..13 {
        println!("{}: I have {} many oranges", x, how_many(x))
    }

    let point = (4, 1);
    match (point) {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis: y = {}", y),
        (x, 0) => println!("y axis: x = {}", x),
        (x, y) => println!("x = {}, y = {}", x, y),
    }

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
        Color::RgbColor(0, 0, 0) | Color::CmykColor { black: 255, .. } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "1 or 2",
        12 => "a dozen",
        9...11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}
