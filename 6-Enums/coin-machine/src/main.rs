enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter
}

fn in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25
	}
}
fn main() {
	let result = in_cents(Coin::Penny);
	print!("{}", result)
}
