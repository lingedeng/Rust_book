fn main() {
	let s = String::from("Hello");
	let len = calc_length(&s);
	println!("The length of '{}' is {}", s, len);
	
	
	let s2 = String::from("Change");
	// types differ in mutability
	// = note: expected mutable reference `&mut String`
	//					  found reference `&String`
	//change(&s2);
	
	// change(&mut s2);
    // ^^^^^^^ cannot borrow as mutable
	//change(&mut s2);
	println!("s2: {}", s2);
	
	let mut s3 = String::from("Change");
	//change_no_mut(&s3);
	change(&mut s3);
	println!("s3: {}", s3);	
	
	//let r1 = &mut s3;
	// ^^^^^^^ second mutable borrow occurs here
	// let r2 = &mut s3;	
	{
		let r1 = &mut s3;
	}
	let r2 = &mut s3;
	//println!("r1: {}, r2: {}", r1, r2);
	
	{
		let r3 = &s3;	// no problem
		let r4 = &s3;	// no problem
		//let r5 = &mut s3;	// big problem
		// -- immutable borrow later used here
		//println!("r3: {}, r4: {}, r5: {}", r3, r4, r5);
		println!("r3: {}, r4: {}", r3, r4);
		
		let r5 = &mut s3; // no problem
		println!("r5: {}", r5);
	}
}

fn calc_length(s: &String) -> usize {
	s.len()
}

fn change(a_str: &mut String) {
	a_str.push_str(", World!");
}

// `a_str` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//fn change_no_mut(a_str: &String) {	
	//a_str.push_str(", World!");
//}