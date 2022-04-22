fn main() {
	let mut s = String::from("Hello Rust!");
	let idx = first_word(&s);
	println!("s: {}, space index: {}", s, idx);
	
	let hello = &s[..5];
	let rust = &s[6..];
	let all = &s[..];
	println!("{}, {}, {}", hello, rust, all);
	
	let hello = first_word_slice(&s);
	println!("first word of '{}' is '{}'", s, hello);
	let hello1 = first_word_slice(&s[..]);
	println!("first word of '{}' is '{}'", s, hello1);
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	
	s.len()
}

fn first_word_slice(s: &str) -> &str {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	
	&s[..]
}