fn main() {
    println!("Hello, world!");

    // let f = another_function(5,6);
    // println!("The value of f is: {:?}", f);
    // let x = five();
    // println!("five is {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// fn another_function(x: i32, y: i32){
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);


//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     let z = println!("The value of y is: {}", y);

//     println!("The value of z is: {:?}", z);
// }

// fn five() -> i32 {
//     5
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}