/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/
mod front_of_house;

mod back_of_house {
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}
	
	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("Apple"),
			}
		}
	}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	
	// absolute path
	crate::front_of_house::hosting::seat_at_table();
	// relative path
	front_of_house::hosting::seat_at_table();
	
	let mut meal = back_of_house::Breakfast::summer("Rye");
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please!", meal.toast);
	
	// The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
	//meal.seasonal_fruit = String::from("Peach");
}