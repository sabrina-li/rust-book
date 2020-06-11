fn main() {
    // let some_u8_value = Some(3u8);
    // match some_u8_value{
    //     Some(3) => println!("three"),
    //     _ => ()
    // }

    // if let Some(3) = some_u8_value{
    //     println!("three")
    // };

    // let another_u8_value = Some(4u8);

    // if some_u8_value == another_u8_value {
    //     println!("three")
    // }
    // if let another_u8_value = some_u8_value{
    //     println!("three")
    // }
    let coin1 = Coin::Quarter(UsStates::Alabama);
    let coin2 = Coin::Penny;

    if let Coin::Quarter(state) = &coin1{
        println!("quarter {:?}", state);
    }else{
        println!("other {:?}",&coin1);
    }

    // println!("{:?}",coin1);

    if let Coin::Quarter(state) = coin2{
        println!("quarter {:?}", state);
    }else{
        println!("other {:?}",coin2);
    }

    // println!("{:?}",coin2);
}

#[derive(Debug)]
enum Coin{
    Quarter(UsStates),
    Penny,
    Dime,
    What,
}

#[derive(Debug)]
enum UsStates{
    Alabama,
    Alaska,
}