enum IpAdd {
    V4,
    V6
}

enum IpAddTwo {
    V4(String),
    V6(String)
}

enum IpAddThree {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr {
    kind: IpAdd,
    address: String
}

struct IpAddrTwo {
    tpy: String,
    address: String,
}

enum IpAddFour {
    V4(IpAddrTwo),
    V6(IpAddr)
}

enum Message {
    Quit,
    Move {x: i32, y: i32}, //struct variant
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {

    fn call(&self) {

        match self {
            Message::ChangeColor(x, y, z) => {
                println!("The Color variant: x {} y {} z {}", x, y, z);
            }
            Message::Move { x, y } => {
                println!("The direction to move: x {} y {}", x, y);
            }
            Message::Quit => {
                println!("Quit the call");
            }
            Message::Write(wrd) => {
                println!("The word is: word {}", wrd);
            }
        }

    }

}

enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

impl Coin {

    fn value_in_cents(&self) -> u8 {

        match self {
            Coin::Penny => {
                println!("lucky");
                1
            }
            Coin::Quarter(state) => {

                match state {
                    UsState::Alabama => println!("Sweet alabama"),
                    UsState::Alaska => println!("Sweet alaska")
                }

                25
            }
            Coin::Nickel => 5,
            Coin::Dime => 10
        }

    }

}

fn value_in_new_cents(coin: Coin) -> u8 {

    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => {
            println!("lucky");
            1
        }
        Coin::Quarter(state) => {

            match state {
                UsState::Alabama => println!("Sweet alabama"),
                UsState::Alaska => println!("Sweet alaska")
            }
            25
        }
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {

    match x {
        None => None,
        Some(i) => Some(i + 1)
    }

}

fn main() {
    let home: IpAddr = IpAddr {
        kind: IpAdd::V4,
        address: String::from("123:6:6:1")
    };

    println!("The value of home {}", home.address);

    let home_two: IpAddTwo = IpAddTwo::V4(String::from("127.0.0.1"));
    let loop_back: IpAddTwo = IpAddTwo::V6(String::from("::1"));

    println!("2. The Value of home_two: ");

    let home_three: IpAddThree = IpAddThree::V4(127, 0, 0, 1);
    let home_four: IpAddThree = IpAddThree::V6(String::from("::1"));

    let ip_addr: IpAddr = IpAddr {
        kind: IpAdd::V4,
        address: String::from("value")
    };

    let ip_addr_two: IpAddrTwo = IpAddrTwo {
        tpy: String::from("value"),
        address: String::from("Farakka")
    };

    let home_five: IpAddFour = IpAddFour::V4(ip_addr_two);
    let home_six: IpAddFour = IpAddFour::V6(ip_addr);

    let m: Message = Message::Write(String::from("hello"));
    m.call();

    let c: Coin = Coin::Dime;
    println!("{}", c.value_in_cents());

    let c_one: Coin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_new_cents(c_one));

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    let seven = match six {
        None => -1,
        Some(s) => s + 1
    };

    println!("{}", seven);
}
