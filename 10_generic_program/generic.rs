fn largest(list: &[i32]) -> i32 {
	let mut largest: i32 = list[0];
	
	//for v in list { // list return type is &i32, so v: &i32
	//for ref v in list { // add ref to list return type, so v: &&i32
	for &v in list { // list return type is &i32, so v: i32
		if v > largest {
			largest = v;
		}
	}
	
	largest
}

/*
fn largestTmpl<T>(list: &[T]) -> T {
	let mut largest = list[0];
	
	for &v in list {
		// Because we want to compare values of type T in the body, 
		// we can only use types whose values can be ordered.
		if v > largest {
			largest = v;
		}
	}
	
	largest
}
*/

struct Point<T> {
	x: T,
	y: T,
}

/*
By declaring T as a generic type after impl, 
Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
*/
// error[E0412]: cannot find type `PointType` in this scope
// impl未加<T>时，编译器会认为Point<T>中的T是一个已定义的具体类型，如果在已知的类型中找到则会报错
//impl Point<PointType> {
impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

/*
means the type Point<f32> will have a method named distance_from_origin and 
other instances of Point<T> where T is not of type f32 will not have this method defined. 
*/
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

struct Point2<T, U> {
	x: T,
	y: U,
}

//Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures.
struct PPoint<X1, Y1> {
	x: X1,
	y: Y1,
}

impl<X1, Y1> PPoint<X1, Y1> {
	fn mixup<X2, Y2>(self, other: PPoint<X2, Y2>) -> PPoint<X1, Y2> {
		PPoint {
			x: self.x,
			y: other.y,
		}
	}
}

// We can use trait bounds to specify that a generic type can be any type that has certain behavior.
// Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
// 在编译时将泛型代码转化针对具体类型的代码，对运行时的效率没有任何影响

// When you need lots of generic types in your code, 
// it could indicate that your code needs restructuring into smaller pieces.
fn main() {
	let list = vec![15, 17, 8, 99, 2, 7];
	let largest = largest(&list);
	
	println!("largest value in list: {}", largest);
	
	let p1 = Point { x: 5, y: 10 };
	let p2 = Point { x: 1.0, y: 4.0 };
	println!("p1.x: {}, p2.x: {}", p1.x(), p2.x());
	// ^^^ expected integer, found floating-point number
	//let p3 = Point { x: 5, y: 4.0 };
	let p3 = Point2 { x: 5, y: 4.0 };
	// field, not a method
	//println!("p3.x: {}", p3.x());
	
	let p4 = Point { x: 3.0, y: 4.0 };
	// why p4 can access x() method?
	/*
	 * because p4 generate two Point implement
	 impl Point<f32> {
		fn x(&elf) -> &f32 { ... }
	 }
	 impl Point<f32> {
		fn distance_from_origin(&self) -> f32 { ...}
	 }
	println!("p4.x: {}, distance from origin: {}", p4.x, p4.distance_from_origin());
	*/	
	
	let pp1 = PPoint { x: 5, y: 10.4 };
	let pp2 = PPoint { x: "hello", y: 'c' };
	
	let pp3 = pp1.mixup(pp2);
	println!("pp3.x = {}, pp3.y = {}", pp3.x, pp3.y);
}