fn main() {
    let ip_4 = IpAddrKind::V4;
    let ip_6 = IpAddrKind::V6;

    println!("{:?}",ip_4);
    println!("{:?}",ip_6);

    let mut string = match_ips(ip_4);
    println!("{}", string);
    string = match_ips(ip_6);
    println!("{}", string);

    // this operation is not allowed because the ip_4 is consumed five lines ago
    // by the match_ips function call
    // let string = match_ips(ip_4);
    // println!("{}", string);

    value_in_cents(Coin::Quarter(UsState::Michigan));

    make_move(1);
}

// creates an enum that can be formatted using the debug formatter
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

// function to match an enum type to its value, then return a string based on match
fn match_ips(ip: IpAddrKind) -> String {
    match ip {
        IpAddrKind::V4 => String::from("That'll work!"),
        IpAddrKind::V6 => String::from("That won't do at all. Not at all."),
    }
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Indiana,
    Michigan,
    Virginia,
    New_York
}

// matches must be exhaustive
// if there is a possible outcome, the match must address it
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// this function accepts a number then uses match to determine behavior
// if the number is 3 or 7, we have specific behavior
// in all other cases, we do nothing
// the _ symbol is a wildcard that does not bind to a value
// the () unit symbol is just an empty unit, it stands for no action
fn make_move(roll_num: u8) {
    match roll_num {
        3 => println!("You get a new hat!"),
        7 => println!("You must turn in one of your hats. Now."),
        _ => (),
    }
}
