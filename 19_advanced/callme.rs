#![crate_type = "dylib"]

#[no_mangle]
pub extern "C" fn call_from_c() {
	println!("Just called a Rust function from C!");
}