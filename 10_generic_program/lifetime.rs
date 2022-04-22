/*
 * every reference in Rust has a lifetime, which is the scope for which that reference is valid. 
 * Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.
 * The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
 * Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. 
 * Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.
 *
 * The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. 
 * Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
 *	The first rule is that each parameter that is a reference gets its own lifetime parameter.
 *	The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters。
 *	The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, 
 *		the lifetime of self is assigned to all output lifetime parameters.
 
 Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
 The annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.
 For example: 
	let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a. 
	The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a. 
	The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.
	
 When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
 lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
 */
 use std::fmt::Display;

// The constraint we want to express in this signature is that 
// the lifetimes of both of the parameters and the lifetime of the returned reference are related 
// such that the returned reference will be valid as long as both the parameters are. 

// it means that the lifetime of the reference returned by the longest function is 
// the same as the smaller of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
// ImportExcerpt实例不能比其part字段的对象活的更久
// 生命周期标识也是泛型的一种，在编译时会使用具体的生命周期对其进行替换，查看是否违反其限制
#[derive(Debug)]
struct ImportExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportExcerpt<'a> {
	// we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
	fn level(&self) -> i32 {
		3
	}
	
	// Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. 
	// Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
	fn announce_and_return_part(&self, announcement: &str) -> &str {
		println!("Attention please: {}", announcement);
		self.part
	}
}

fn longest_with_an_announcement<'a, T>(
	x: &'a str,
	y: &'a str,
	ann: T,
) -> &'a str
where
	T: Display {
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
	// One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program.
	let s: &'static str = "I have a static lifetime!";
	
	/*
	 * ^^ borrowed value does not live long enough
	{
		let r;
		{
			let x = 5;
			r = &x;
		}
		println!("r: {}", r);
	}
	*/
	
	// When we pass concrete references to longest, the concrete lifetime that 
	// is substituted for 'a is the part of the scope of x that overlaps with the scope of y. 
	// In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
	let s1 = String::from("whoami");
	let s2 = "xyz";
	let r1 = longest(s1.as_str(), s2);
	println!("longest str:{}", r1);
	
	{
		let s3 = String::from("Hi");
		let r2 = longest(s3.as_str(), s2);
		println!("longest str:{}", r2);
	}
	
	/*
	 * `s4` does not live long enough
	let r3;
	{
		let s4 = String::from("Ok");
		r3 = longest(s4.as_str(), s2);		
	}
	println!("longest str:{}", r3);
	*/
	
	let novel = String::from("Call me Ishmael. Some years ago...");	
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let i = ImportExcerpt {
		part: first_sentence,
	};
	println!("i: {:?}", i);
}