fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s: {}", s);
	
	let x = 5;
	let y = x;
	println!("x: {}, y: {}", x, y);
	
	let s1 = String::from("whoami");
	//let s2 = s1; 
	let s2 = s1.clone();
	println!("s1: {}, s2: {}", s1, s2);
	
	
	let s3 = String::from("I'm dead");
	take_ownership(s3);
	//println!("s3: {}", s3); //value borrowed here after move
	let z = 10;
	take_copy(z);
	println!("z: {}", z);
	
	let s4 = gives_ownership();
	let s5 = String::from("back to me!");
	let s6 = takes_and_gives_back(s5);
	//println!("s5: {}", s5); //value borrowed here after move
	println!("s4: {}, s6: {}", s4, s6);
}

fn take_ownership(some_string: String) {
	println!("some_string: {}", some_string);
}

fn take_copy(some_integer: i32) {
	println!("some_integer: {}", some_integer);
}

fn gives_ownership() -> String {
	let str = String::from("I'm back!");
	str
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}
