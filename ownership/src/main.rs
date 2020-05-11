fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world");
    // println!("{}", s);
    
    // let mut s1 = String::from("test1");
    // // let s2 = s1;
    // let s2 = s1.clone();

    // s1.push_str("push_str");
    // println!("s1 {}, s2 {}", s1, s2);

    // let s = String::from("hello");
    // takes_ownership(s);

    // let x = 5;
    // makes_copy(x);

    // // println!("{}", s); // <- err
    // println!("{}", x);

    // let s1 = gives_ownership();

    // let mut s2 = String::from("hello2");
    
    // let s3 = takes_and_gives_back(s2);

    // // s2 = s1;
    // println!("{}",s1);
    // // println!("{}",s2); // <- err
    // println!("{}",s3);
    // s2 = s1;

    let s1 = String::from("hello!");
    let (s1,len) = calculate_length(s1);
    println!("{},{}",s1,len);
}

// fn takes_ownership(some_string: String){
//     println!("{}",some_string);
// }

// fn makes_copy(some_integer: i32){
//     println!("{}",some_integer);
// }

// fn gives_ownership() -> String{
//     let s = String::from("hello");
//     s
// }

// fn takes_and_gives_back(some_string: String) -> String {
//     some_string
// }

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();
    // (s, s.len())
    (s,length)
}