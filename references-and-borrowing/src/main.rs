fn main() {
    // let mut s = String::from("hello");

    // let len = calculate_length(s1)
    // change(&mut s1);
    // println!("{}", s1);
    // {
    //     let r1 = &mut s1;
    // }
    // let r2 = &mut s1;

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // println!("{}, {}", r1, r2);

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // r1 and r2 are no longer used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // <- s deallocate, err
}



// fn calculate_length(s: String) -> usize {
//     s.push_str("test");
//     s.len()
// }

// fn change(s: &mut String) {
//     s.push_str(", world");
// }