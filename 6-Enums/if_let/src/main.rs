use std::fmt::format;

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState)
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return  None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old"))
    }
    else {
        Some(format!("{state:?} is relatively new"))
    }
}

fn main() {

    let coin = Coin::Quarter(UsState::Alaska);
    let result = describe_state_quarter(coin).unwrap();

    println!("{result}")
}
