fn constants() {
    // Shodowing
    let x = 5;

    let x = x + 1;

    {
        let x = x + 1;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn arrays() {
    let arr = ["January", "February", "March", "April", "May", "June", "July", 
                            "August", "September", "October", "November", "December"];

    println!("{:?}", arr);
}


//  The string type

fn strings() {
    let _s = String::from("Hello"); // The double colon :: operator allows us to namespace this particular  "from" function inder
    // the String type rather than using some sort of name like string_from


    // This kind of string can be muted
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s); // This will print  `Hello world`

    // Variables and Data Interacting with Clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {} , s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn change (some_string: &mut String){
    some_string.push_str(", world!");
}

// The slice type
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    constants();

    let guess: u32 = "23".parse().expect("Not a number");

    println!("Guessed number: {guess}");

    arrays();

    strings();

    let s = String::from("Hello, 254");

    // Takes the ownership of string S
    takes_ownership(s);

    // println!("{}", s); Results in an error

    let mut s1 = String::from("Hello");

    change(&mut s1);

    println!("string 1 => {s1}"); 

    let  s = String::from("Hello World");
    let word = first_word(&s);


    println!("{word}")
}
