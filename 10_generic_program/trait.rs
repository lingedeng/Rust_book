/*
 * Different types share the same behavior if we can call the same methods on all of those types. 
 * Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
 * Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
 *
 * One restriction to note with trait implementations is that 
 * 	we can implement a trait on a type only if at least one of the trait or the type is local to our crate. 
 * But we can’t implement external traits on external types.
 
 One restriction to note with trait implementations is that 
 we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
 
 This restriction is part of a property of programs called coherence, 
 and more specifically the orphan rule, so named because the parent type is not present.
 
 Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
 For example:
	the standard library implements the ToString trait on any type that implements the Display trait. 
	impl<T: Display> ToString for T {
		// --snip--
	}
	Borrow has a blanket impl for any T, but AsRef has not a blanket impl for any T
	impl<T: ?Sized> const Borrow<T> for T {    
		fn borrow(&self) -> &T {
			self
		}
	}
Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also 
specify to the compiler that we want the generic type to have particular behavior.
The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.
 */
 
 trait Summary {	
	fn summarize_author(&self) -> String;
	
	fn summarize(&self) -> String {
		format!("(Read more from {}...)", self.summarize_author())
	}	
	/*
	fn summarize(&self) -> String {
		String::from("(Read more...)")
	}
	*/
 }
 
 struct News {
	headline: String,
	location: String,
	author: String,
	content: String,
 }
 
 impl Summary for News {
	fn summarize_author(&self) -> String {
		format!("Author: {}", self.author)
	}
 }
 
 /*
 impl Summary for News {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
 }
 */
 
 struct Tweet {
	username: String,
	content: String,
	reply: bool,
	retweet: bool,
 }
 
 impl Summary for Tweet {
	/*
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
	*/
	fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
 }
 
// This parameter accepts any type that implements the specified trait.
// '&impl trait' syntax as syntax sugar for trait bound
//fn notify(item: &impl Summary) {
// 表示泛型类型绑定了Summary特性
fn notify<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
//fn notify(item: &(impl Summary + Display)) { ... }
//fn notify<T: Summary + Display>(item: &T) { ... }

// Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. 
fn largest<T>(list: &[T]) -> T 
	where T: PartialOrd + Copy {
	let mut largest = list[0];
		
	for &item in list {
		if item > largest {
			largest = item;
		}
	}
		
	largest
}

fn largest_ex<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
	
	for item in list {
		if item > largest {
			largest = item;
		}
	}
	
	largest
}

// 如果需要多种返回绑定特性的类型，则这种写法无法工作
// see “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17
fn returns_summarizable() -> impl Summary {
// error[E0106]: missing lifetime specifier
//fn returns_summarizable() -> &impl Summary {
// error[E0308]: mismatched types
// ^ expected `&T`, found struct `Tweet`
//fn returns_summarizable<T: Summary>() -> &'static T {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


 
 fn main() {
	let tweet = Tweet {
		username: String::from("eric"),
		content: String::from("Virus Omi"),
		reply: false,
		retweet: false,
	};
	
	println!("1 new tweet: {}", tweet.summarize());
	
	let news = News {
		headline: String::from("Happy new year"),
		location: String::from("NJ"),
		author: String::from("eric"),
		content: String::from("Happy chinese new year!"),
	};
	
	println!("New article available! {}", news.summarize());
	
	notify(&tweet);
	notify(&news);
	
	let list = vec![10, 17, 8, 9, 2, 100];
	println!("largest value in list: {}", largest(&list));
	println!("largest_ex value in list: {}", largest_ex(&list));
	let list_char = vec!['m', 'q', 'z', 'a'];
	println!("largest value in list_char: {}", largest(&list_char));
	println!("largest_ex value in list_char: {}", largest_ex(&list_char));
	
	// error[E0308]: mismatched types
	// expected `i32`, found opaque type
	//let t1: i32 = returns_summarizable();
	let t1 = returns_summarizable();
	// error[E0308]: mismatched types
	// expected reference, found opaque type
	//notify(t1);
	notify(&t1);
	
 }