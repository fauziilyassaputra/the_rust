enum IpAddKind {
    V4, V6
}

struct IpAdd {
    kind: IpAddKind,
    address: String
}

enum Message {
    Quit,
    Move {x: i32,y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

// impl with enum
impl Message {
    fn call(&self){
    }
}


// enum option ( the concept of a value being present or absent )

// enum Option<T>{
//     None,
//     Some(T)
// }

// match


#[derive(Debug)]
enum UsState {
    Alamaba,
    Alaska
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alamaba => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}


fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900){
            Some(format!("{state:?} is pretty old for america"))
        }
        else {
            Some(format!("{state:?} is  relatively new"))
        }
    } else { None }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}



#[allow(unused)]
fn main() {
    let home = IpAdd {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAdd {
        kind: IpAddKind::V6,
        address: String::from("::1")
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    let absent_number: Option<i32> = Option::None;


    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("the result: {:?} , None: {:?}", six,none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()

    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}


    // if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        // println!("The maximum is configured to be max {}",)
    }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
}
