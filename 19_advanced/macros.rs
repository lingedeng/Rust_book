/// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
///		Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
///		Attribute-like macros that define custom attributes usable on any item
///		Function-like macros that look like function calls but operate on the tokens specified as their argument
///
/// you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

#[macro_export]
macro_rules! myvec {
	// macro patterns are matched against Rust code structure rather than values
	($( $x:expr ), *) => {
		{
			let mut tmp_vec = Vec::new();
			$(
				tmp_vec.push($x);
			)*
			tmp_vec
		}		
	};
}

fn declarative_macro() {
	println!("~declarative macro~");
	
	let v = myvec![1, 2, 3];	
	assert_eq!(v, vec![1, 2, 3]);
	
}
/// Procedural macros accept some code as an input, operate on that code, and produce some code as an output 
/// rather than matching against patterns and replacing the code with other code as declarative macros do.

fn main() {
	declarative_macro();
	todo!("Procedural macros");
}