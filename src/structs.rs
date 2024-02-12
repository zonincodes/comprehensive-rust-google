// Defining a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: User){
    println!(
        "Username: {} \nEmail {} \nActive {} \nSign_in_count {}",
        user.username, user.email, user.active, user.sign_in_count
    );
}

// using tuples struct without named fields to create diffent types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    // creating an instance
    let mut user1 = User {
        active: true,
        username: String::from("someoneuser"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,


    };

    let user2 = build_user(
        String::from("someoneuser"),
        String::from("someone@example.com"),
    );

    user1.email = String::from("anotheremil@example.com");

    println!("{}", user2.sign_in_count);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };

    print_user(user3);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    
    println!("color: {} point: {}", black.0, origin.1);
}
// Defining a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: User){
    println!(
        "Username: {} \nEmail {} \nActive {} \nSign_in_count {}",
        user.username, user.email, user.active, user.sign_in_count
    );
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}


fn main() {
    // creating an instance
    let mut user1 = User {
        active: true,
        username: String::from("someoneuser"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,


    };

    let user2 = build_user(
        String::from("someoneuser"),
        String::from("someone@example.com"),
    );

    user1.email = String::from("anotheremil@example.com");

    println!("{}", user2.sign_in_count);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };

    print_user(user3);

    let rect1 = Rectangle{
        width: 10,
        height: 20,
    };

    println!("{:#?}", rect1);

    let scale = 2;
    let rect2: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    // 
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    
}
