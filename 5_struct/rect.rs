struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(self: &Self) -> u32 {
		self.width * self.height
	}
	
	fn can_hold(&self, other: &Self) -> bool {
	//fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
	
	fn square(size: u32) -> Self {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn main() {
	let rect = Rectangle {
		width: 30,
		height: 50,
	};
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};
	
	println!("the rectangle area is {}", rect.area());
	println!("the rect can hold rect2? {}", rect.can_hold(&rect2));
	println!("the rect can hold rect3? {}", rect.can_hold(&rect3));
	
	let sq = Rectangle::square(3);
	println!("the square area is {}", sq.area());
}