fn main() {
// scalar
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("guess is: {}", guess)
    // let num: u8 = 0;
    // let num = num - 1;
    // println!("num is {}", num);

    // let x = 2.1; // f64
    // let y: f32 = 3.0; // f32
    // println!("x,y is {},{}", x, y);

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;

    // // remainder
    // let remainder = 43 % 5;

    // println!("results are {}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);


    // let t = true;

    // let f: bool = false; // with explicit type annotation

    // println!("results are {}, {}", t, f);

    // let c: &str = "z";
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';
    // println!("results are {}, {}, {}", c, z, heart_eyed_cat);

//compound 
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("tople is {:?}", tup);
    // println!("x,y,z is {}, {}, {}",x ,y, z);

    // let five_hundred = tup.0;

    // let six_point_four = tup.1;

    // let one = tup.2;

    // println!("access tuple: {} {} {}", five_hundred, six_point_four, one)

    let a: [u32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
            //   "August", "September", "October", "November", "December"];
    // println!("array is {:?}", a);
    
    // let first = a[0];
    // let second = a[1];
    // println!("first and second: {}, {}", first, second);

    let index = 10;

    let element = a[index];
    // panick instead of accessing unsafe memory
    println!("The value of element is: {}", element);
}