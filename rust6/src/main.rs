
// regular enums
enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// you can add methods to enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
// enum that takes a string
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    println!("home {:?}", home);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("loopback {:?}", loopback);
    // option used in place of null
    // T works like c# or C++
    let absent_number: Option<i32> = None;
    println!("absent_number {:?}", absent_number);
    let absent_number = Some(3);
    println!("absent_number {:?}", absent_number);
    println!("absent_number {:?}", absent_number.unwrap());
    
}
