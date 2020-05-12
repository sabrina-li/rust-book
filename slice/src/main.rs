fn main() {
    let mut s = String::from("hello world");

    // let f = first_word(&s);

    // s.clear();

    // println!("{}, {}", s, f);

     // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{},{}", hello, world);

    let f = first_word(&s);
    println!("{}", f);

    s.clear();
    // println!("{}", f);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate(){
//         if item== b' '{
//             return i
//         }
//     }

//     s.len()
// }


fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    &s[..]
}