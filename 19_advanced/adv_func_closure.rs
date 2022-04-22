/// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), 
/// so you can always pass a function pointer as an argument for a function that expects a closure.
/// It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.
///
/// An example of where you would want to only accept fn and not closures is 
/// when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

fn add_one(x: i32) -> i32 {
	x + 1
}

fn do_twice(arg: i32, f: fn(i32) -> i32) -> i32 {
	f(arg) + f(arg)
}

fn func_pointer() {
	println!("~func pointer~");
	let num = do_twice(5, add_one);
	
	assert_eq!(num, 12);
}

fn func_closure() {
	println!("~func closure~");
	let v1 = vec![1, 2, 3];
	let v1_str: Vec<String> = v1.iter().map(|i| i.to_string()).collect();
	
	let v2 = vec![4, 5, 6];
	// we’re using the to_string function defined in the ToString trait, 
	// which the standard library has implemented for any type that implements Display.
	let v2_str = v2.iter().map(ToString::to_string).collect::<Vec<String>>();
	
	assert_eq!(v1_str, vec!["1", "2", "3"]);
	assert_eq!(v2_str, vec!["4", "5", "6"]);
}

/// you can’t do that with closures because they don’t have a concrete type that is returnable; 
/// you’re not allowed to use the function pointer fn as a return type
// error[E0746]: return type cannot have an unboxed trait object
//fn add_one_closure() -> dyn Fn(i32) -> i32 {
/// 函数中使用let，则需在函数调用栈中建立了一个closure类型；当函数返回时，在栈中建立的closure类型会被销毁，则无法返回其引用
/// error[E0515]: cannot return reference to local variable `c`
/// 	returns a reference to data owned by the current function
/// ```
/// fn add_one_closure_1<'a>() ->  &'a dyn Fn(i32) -> i32 {
///		let c = |x| x + 1;
///		&c
///	}
/// ```

/// 函数中未使用let，则在程序的静态区(.data)中建立一个closure类型；当函数返回时，返回的是对静态区closure类型的引用
fn add_one_closure_2<'a>() -> &'a dyn Fn(i32) -> i32 {
	&|x| x + 1
}

/// 更通常的做法是使用smart pointer对其进行封装
fn add_one_closure_3() -> Box<dyn Fn(i32) -> i32> {
	Box::new(|x| x + 1)
}

/// 下面的示例与返回closure时的机制相同
/// ```
/// fn return_ref_1<'a>() -> &'a i32 {
///		let i = 2;
///		&i
/// }
/// ```

fn return_ref_2<'a>() -> &'a i32 {
	&2
}

fn return_closure() {
	println!("~return closure~");	
	
	let f1 = add_one_closure_3();
	assert_eq!(f1(1), 2);
}

fn main() {
	func_pointer();
	func_closure();
	return_closure();
}