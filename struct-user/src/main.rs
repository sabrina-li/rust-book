fn main() {
    println!("Hello, world!");

    let mut user = User {
        username: String::from("user_i"),
        email: String::from("test@gmail.com"),
        sign_in_count: 2,
        active: true,
    };

    user.email = String::from("test");

    println!("email: {}",build_user(String::from("test@gmail.com"),String::from("user_i")).email);


    let user2 = User{
        email:String::from("email2"),
        username: String::from("user2"),
        // sign_in_count:user.sign_in_count,
        // active:user.active,
        ..user
    };


    //tuple struct
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let Color(a,b,c) = black;

    println!("{}, {}, {}",a,b,c);
}

fn build_user(email:String, username:String) -> User{
    User{
        username,
        email,
        sign_in_count: 2,
        active: true,
    }
}


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}