fn if_statement() {
    let temp = 35;
    if temp > 30 {
        println!("it's hot");
    } else if temp < 10 {
        println!("it's chilly");
    } else {
        println!("it's moderate");
    }

    // if can also be an expression
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);
    println!(
        "it is {}",
        if temp > 20 {
            "hot"
        } else if temp < 20 {
            "cold"
        } else {
            "cold"
        }
    );
    println!(
        "it is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    )
}

fn while_loop() {
    // while condition == true
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x = {}", x)
    }

    // while true
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    // range over an array
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x)
    }

    // enumerate over an array
    for (pos, y) in (30..41).enumerate() {
        println!("pos = {}, y = {}", pos, y);
    }
}

fn match_statement() {
    // ... inclusive range
    // .. exclusive range
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweeden",
        7 => "Russia",
        1...999 => "unknown", // matches only allow inclusive ranges
        _ => "invalid",
    };
    println!("the country with code {} is {}", country_code, country);
}

fn main() {
    if_statement();
    while_loop();
    for_loop();
    match_statement();
}
