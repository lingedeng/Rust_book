#[derive(Debug)]
enum UsState {
	Alabama,
    Alaska,
	CA,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky Penny!");
			1
		}
		// if commented Coin::Nickel arm, ^^^^ pattern `Nickel` not covered
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}!", state);
			25
		}
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i+1),
	}
}

fn add_fancy_hat() {
	println!("add fancy hat!");
}

fn remove_fancy_hat() {
	println!("remove fancy hat!");
}

fn move_player(steps: u8) {
	println!("move player {} steps!", steps);
}

fn reroll() {
	println!("reroll!");
}


fn main() {	
	let cents = value_in_cents(Coin::Penny);	
	println!("insert coin in cents: {}", cents);
		
	let cents = value_in_cents(Coin::Quarter(UsState::CA));	
	println!("insert coin in cents: {}", cents);
	
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
	println!("six: {:?}, none: {:?}", six, none);
	
	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other_value => move_player(other_value),
		//_ => reroll(), // We aren’t going to use the value
		//_ => (), // do nothing
	}

	// you can think of if let as syntax sugar for a match 
	// that runs code when the value matches one pattern and then ignores all other values.
	// We can include an else with an if let.
	let config_max = Some(8u8);
	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	} else {
		println!("The maximum is None");
	}
}