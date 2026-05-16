fn main() {

    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("username@example.com"),
        sign_with_count: 1
    };
    println!("{:?}", user1);


     let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("username@example.com"),
        sign_with_count: 1
    };
    user1.username = String::from("mahattancafe");
    println!("{:?}", user1);


    // creating instance with update syntax

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("username2@example.com"),
    //     sign_with_count: user1.sign_with_count,
    // };

    // or

    let user2 = User {
        email: String::from("username"),
        ..user1
    };

    // Refactor with no struct

    let height = 30;
    let width = 50;


    println!(
        "The area of rectangle is: {}",
        area(width,height)
    );

    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_d(rect1)
    );

    // refactor with struct

    let rect2 = Rectangle {
        width: 30,
        height:50
    };

    println!("rectangle with struct: {}",
    area_struct(&rect2)
    );

    // dbg!

    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(rect4);

}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_with_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // username:username
        email, // email: email
        sign_with_count: 1
    }
}

fn area(width: u32, height: u32) -> u32{
    width * height
}

fn area_d(dimension: (u32,u32)) -> u32{
    dimension.0 * dimension.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}