struct Foo {
    x: (u32, u32),
    y: u32,
}


enum Results{
    Ok(i32),
    Err(String)
}


fn divide_in_two(n: i32) -> Results {
    if n % 2 == 0 {
        Results::Ok(n / 2)
    } else {
        Results::Err(format!("cannot divide {n} in two equal parts"))
    }
}

fn dest_struct() -> () {
    let foo = Foo { x: (0, 1), y:3};

    match foo {
        Foo {x: (1, b), y}  => println!("x.0 = 1, b={b}, y = {y}"),
        Foo {y: 2, x: i} => println!("y = 2, x={i:?}"),
        Foo {y, ..} => println!("y = {y}, other fields were ignored")
    }
}



// If let expressions
fn sleep_for(secs: f32){
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };

    std::thread::sleep(dur);
    println!("Slept for {:?}", dur);
}

// Let else Expressions
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String>{
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Err(String::from("got empty string"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("not a hex digit"))
    }
}

fn main() {
    dest_struct();


    let n = 100;
    match divide_in_two(n) {
        Results::Ok(half)  => println!("{n} divided in two is {half}"),
        Results::Err(msg) => println!("Sorry, an error happened: {msg}"),
    }

    sleep_for(-10.0);
    sleep_for(0.8);

    println!("result: {:?} ", hex_or_die_trying(Some(String::from(""))));

    let mut name = String::from("Comprehensive Rust 🦀");

    while let Some(c) = name.pop() {
        println!("Charater: {c}");
    }
}