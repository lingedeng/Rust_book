/// trait associated types VS trait generics
/// when a trait has a generic parameter, it can be implemented for a type multiple times, 
///	changing the concrete types of the generic type parameters each time.
///	When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.
///
///	With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.
/// 

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

impl std::ops::Add for Point {
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

fn trait_associated_types() {
	println!("~trait associated types~");
	
	assert_eq!(
		Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
		Point { x: 3, y: 3 }
	);
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

/// std::ops::Add trait defined:
/// ```
/// pub trait Add<Rhs = Self> {
///    type Output;
///    fn add(self, rhs: Rhs) -> Self::Output;
///	}
/// ```
/// The new part is Rhs=Self: this syntax is called default type parameters.
///
/// You’ll use default type parameters in two main ways:
///		To extend a type without breaking existing code
///		To allow customization in specific cases most users won’t need (Add use ways)
// The syntax for specifying a default type for a generic type is <PlaceholderType=ConcreteType> when declaring the generic type.
impl std::ops::Add<Meters> for Millimeters {
	type Output = Self;
	
	fn add(self, rhs: Meters) -> Self::Output {
		Millimeters(self.0 + (rhs.0 * 1000))
	}
}

fn trait_default_type_param() {
	println!("~trait default type param~");
	
	assert_eq!(
		Millimeters(1000) + Meters(2),
		Millimeters(3000)
	);
}

trait Pilot {
	fn fly(&self);
}

trait Wizard {
	fn fly(&self);
}

struct Human;

impl Pilot for Human {
	fn fly(&self) {
		println!("This is your caption speaking!");
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("Up!");
	}
}

impl Human {
	fn fly(&self) {
		println!("*waving arms furiously*");
	}
}

trait Animal {
	fn baby_name() -> String;
}

struct Dog;

impl Dog {
	fn baby_name() -> String {
		String::from("Spot")
	}
}

impl Animal for Dog {
	fn baby_name() -> String {
		String::from("Puppy")
	}
}

struct Cat;
impl Animal for Cat {
	fn baby_name() -> String {
		String::from("Meow")
	}
}

/// fully qualified syntax
/// <Type as Trait>::function(receiver_if_method, next_arg, ...);
fn call_methods_with_same_name() {
	println!("~call method with same name~");
	let human = Human;
	// Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call. 
	Pilot::fly(&human);
	<Human as Wizard>::fly(&human);
	human.fly();
	Human::fly(&human);
	
	println!("A baby dog is called a {}", Dog::baby_name());
	// error[E0283]: type annotations needed
	// Rust can’t figure out which implementation of Animal::baby_name we want. (Dog or Cat)
	//println!("A baby dog is called a {}", Animal::baby_name());
	// indicates we want to call the baby_name method from the Animal trait as implemented on Dog by 
	// saying that we want to treat the Dog type as an Animal for this function call.
	println!("A baby dog is called a {}", <Dog as Animal>::baby_name());	
	println!("A baby cat is called a {}", <Cat as Animal>::baby_name());
}

/// the orphan rule that states we’re allowed to implement a trait on a type as long as either the trait or the type are local to our crate. 
/// It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct. 
///
/// The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding.
/// 	We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0
///		If we wanted the new type to have every method the inner type has, implementing the Deref trait on the Wrapper to return the inner type
///		we would have to implement just the methods we do want manually
struct Wrapper(Vec<String>);

/*
impl Wrapper {
	fn len(&self) -> usize {
		self.0.len()
	}
}
*/
/// ?Sized means “T may or may not be Sized”
/// The ?Trait syntax with this meaning is only available for Sized, not any other traits.
impl std::ops::Deref for Wrapper {
	type Target = Vec<String>;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::fmt::Display for Wrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "[{}]", self.0.join(", "))
	}
}

fn newtype_pattern() {
	println!("~newtype pattern~");
	let wv = Wrapper(vec![String::from("hello"), String::from("world")]);
	
	println!("wv = {}", wv);
	assert_eq!(wv.len(), 2);
	for (i, s) in wv.iter().enumerate() {
		println!("{}: {}", i, s);
	}
}

fn main() {
	trait_associated_types();
	trait_default_type_param();
	call_methods_with_same_name();
	newtype_pattern();
}