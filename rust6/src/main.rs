
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
    

    //6.2
    let coin = Coin::Penny;
    println!("{:?} value is {:?}", coin, value_in_cents(&coin));
    let coin = Coin::Quarter(UsState::Alabama);
    println!("{:?} value is {:?}", coin, value_in_cents(&coin));
}

#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
