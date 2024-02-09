
fn fib(n: u32) -> u32{
    if n <= 2 {
        return  1;
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fn while_loop() -> (){
    let mut x = 200;
    while x > 10{
        x  /= 2;
    }

    println!("Final x: {x}");
}

fn for_loop() -> () {
    for x in 1..5 {
        println!("x: {x}");
    }
}

fn break_continue(){
    'outer: for x in 1..5{
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3{
                break 'outer;
            }
        }
    }
}



fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1{
        n = if n % 2 == 0 {n / 2} else {3 * n + 1};
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() -> () {
    assert_eq!(collatz_length(11), 15);
}

fn arrays() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");

}

fn array_iteration() -> () {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime{
            assert_ne!(prime % i, 0)
        }
    }
}

fn pattern_matching(input: char) -> () {

    match input {
       'q' => println!("Quitting"),
       'a' | 's' | 'w' | 'd' => println!("Moving around"),
       '0' ..='9' => println!("Number input"),
       key if key.is_lowercase() => println!("Lowercase: {key}"),
       _ => println!("Something else"),
    }
}

fn desciribe_point(point: (i32, i32)) {
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below X axis"),
        _ => println!("first quadrant"),
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_mat : [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3{
        for j in 0..3{
            new_mat[j][i] = matrix[i][j];
        }
    }
    new_mat
}


#[test]
fn test_transpose () {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301,302, 303]
    ];

    let ans: [[i32; 3]; 3] = [
        [101, 201, 301],
        [102, 202, 302],
        [103, 203, 303]
    ];

    assert_eq!(transpose(matrix), ans)
}

fn shared_refernces() -> () {
    let a: char = 'A';
    let b: char = 'B';

    let mut r: &char = &a;
    println!("r {}", *r);

    r = &b;
    println!("r: {}", *r);
}


fn magnitude(point: &[f64; 3]) -> f64 {
    let mut sq: f64 = 0.0;

    for coord in point{
        sq += coord * coord;
    }

    sq.sqrt()
}

fn normalize (vector: &mut [f64; 3]){
    let mag = magnitude(vector);
    vector[0] /= mag;
    vector[1] /= mag; 
    vector[2] /= mag; 
}

struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn test_struct()
{
    let mut peter = Person {name: String::from("Peter"), age: 27};

    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person {name, age};
    describe(&avery);

    let jackie = Person{name: String::from("Jackie"), ..avery};
    describe(&jackie);
}

#[derive(Debug)]
enum Direction {
    Left,
    // Right,
}

#[derive(Debug)]
enum PlayerMove {
    // Pass,                       // Simple variant
    Run(Direction),             // Tupple variant
    // Teleport {x: u32, y: u32},  // Struct variant
}

// const

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    println!("Hello, world!");
    let n: u32 = 20;
    println!("fib(n) = {}", fib(n));

    while_loop();
    for_loop();
    break_continue();
    collatz_length(11);
    arrays();
    array_iteration();
    pattern_matching('x');
    pattern_matching('9');
    pattern_matching('K');
    pattern_matching('s');

    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301,302, 303]
    ];

    transpose(matrix);
    desciribe_point((1, 0));

    shared_refernces();

     println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

    test_struct();
    
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);

    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
