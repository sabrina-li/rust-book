// use std::net::IpAddr;

fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // route(home);
    // route(loopback);

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
}

// fn route(ip: IpAddr){
//     println!("{:?}, {}", ip.kind, ip.address);
// }

// fn route(ip_kind: IpAddrKind){
//     println!("{:?}",ip_kind);
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }


// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String)
// }

#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}