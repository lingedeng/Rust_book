// both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind.
#[derive(Debug)]
enum IpAddrKind {
	V4,
	V6,
}

// the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
#[derive(Debug)]
enum IpAddrEnum {
	V4(u8, u8, u8, u8),
	V6(String),
}

// Defining an enum with variants is similar to defining different kinds of struct definitions
// But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages
// As Message enum defined, which is a single type.
// just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(u32, u32, u32),
}

struct IpAddr {
	kind: IpAddrKind,
	addr: String,
}

fn main() {
	let home = IpAddr {
		kind: IpAddrKind::V4,
		addr: String::from("127.0.0.1"),
	};
	
	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		addr: String::from("::1"),
	};
	
	println!("home addr, kind: {:?}, addr: {}", home.kind, home.addr);
	
	let home1 = IpAddrEnum::V4(127, 0, 0, 1);
	let loopback1 = IpAddrEnum::V6(String::from("::1"));
	println!("home1 addr: {:?}", home1);
	
	let x: i8 = 5;
	let y: Option<i8> = Some(5);
	// ^^^^ cannot infer type for type parameter `T` declared on the enum `Option`
	//let z = None;
	let z: Option<i8> = None;
	//  ^ no implementation for `i8 + Option<i8>`
	//let sum = x + y;
}