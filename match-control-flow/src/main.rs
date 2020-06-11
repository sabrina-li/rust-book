fn main() {


    let coin = Coin::Quarter(UsStates::Alabama);

    println!("{}",value_in_cents(Coin::Quarter(UsStates::Alaska)));
    value_in_cents(coin);

    // println!("{:?}",add_one(Option::Some(32)));
    // println!("{:?}",add_one(Option::None));

    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);

    println!("{:?} {:?} {:?}",five,six,none);

    let some_u8_value = 9u8;
    match some_u8_value{
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }
}

#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}


// impl Coin{
//     fn value_in_cents(&self) -> u8{
//         let res = match self{
//             Coin::Penny => {
//                 println!("lucky penny");
//                 1//return value
//             },
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter => 25,
//         };
//         res
//     }
// }
fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("lucky penny");
            1//return value
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is from {:?}",state);
            25
        },
    }
}


fn add_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(n) => Some(n+1),
    }
}