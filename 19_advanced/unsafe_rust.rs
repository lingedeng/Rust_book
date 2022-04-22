/// Unsafe superpowers include the ability to:
///		Dereference a raw pointer
///		Call an unsafe function or method
///		Access or modify a mutable static variable
///		Implement an unsafe trait
///		Access fields of unions
/// The unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety.

/// Different from references and smart pointers, raw pointers:
///		Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
///		Aren’t guaranteed to point to valid memory
///		Are allowed to be null
///		Don’t implement any automatic cleanup


fn raw_pointer() {
	println!("~raw pointer~");
	let mut num = 5;
	
	let p1: *const i32 = &num;
	let p2: *mut i32 = &mut num;
	
	unsafe {
		println!("p1: {}", *p1);
		println!("p2: {}", *p2);
	}
}

/// The unsafe keyword in this context indicates the function has requirements we need to uphold when we call this function, 
/// because Rust can’t guarantee we’ve met these requirements.
///
/// Bodies of unsafe functions are effectively unsafe blocks, 
/// so to perform other unsafe operations within an unsafe function, we don’t need to add another unsafe block.
unsafe fn dangerous() {
	println!("unsafe function");
}

fn call_unsafe_fn() {
	println!("~call unsafe fn~");
	
	unsafe {
		dangerous();
	}
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
	let len = slice.len();
	let ptr = slice.as_mut_ptr();
	
	assert!(mid <= len);
	
	// error[E0499]: cannot borrow `*slice` as mutable more than once at a time
	//(&mut slice[..mid], &mut slice[mid..])
	
	unsafe {
		(
			std::slice::from_raw_parts_mut(ptr, mid),
			std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)
		)
	}
}

fn safe_abstraction_over_unsafe() {
	println!("~safe abstraction over unsafe~");
	let mut v = vec![1, 2, 3, 4, 5, 6];
	let r = &mut v[..];
	
	let (a, b) = r.split_at_mut(3);
	//let (a, b) = split_at_mut(r, 3);
	
	println!("a: {:?}", a);
	println!("b: {:?}", b);
}

extern "C" {
	fn abs(input: i32) -> i32;
}

fn unsafe_ffi() {
	println!("~unsafe FFI~");
	
	unsafe {
		println!("abs value of -3 according to C: {}", abs(-3));
	}
}

static mut COUNTER: i32 = 0;

fn add_to_counter(inc: i32) {
	unsafe {
		COUNTER += inc;
	}
}

/// Accessing and modifying mutable static variables is unsafe.
fn unsafe_mut_static_access() {
	println!("~unsafe mut static access~");
	
	add_to_counter(3);
	unsafe {
		println!("COUNTER: {}", COUNTER);
	}
}

/// A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
/// By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.
unsafe trait Foo {
	fn foo(&self);
}

// error[E0200]: the trait `Foo` requires an `unsafe impl` declaration
//impl Foo for i32 {
unsafe impl Foo for i32 {
	fn foo(&self) {
		println!("unsafe Foo trait, i32 value: {}", self);
	}
}

fn unsafe_trait() {
	println!("~unsafe trait~");
	
	let i = 32;
	i.foo();
	//<i32 as Foo>::foo(&i);
}

fn main() {
	raw_pointer();
	call_unsafe_fn();
	safe_abstraction_over_unsafe();
	unsafe_ffi();
	unsafe_mut_static_access();
	unsafe_trait();
}