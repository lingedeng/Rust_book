use std::collections::HashMap;

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn new(x: i32, y: i32) -> Point {
		Point {
			x: x,
			y: y,
		}
	}
}

#[derive(Debug)]
struct Student {
	name: String,
	age: u8,
}

fn find_team_score(scores: &HashMap<String, i32>, team: &String) {
	let score = scores.get(team);
	match score {
		Some(s) => println!("team: {}, score: {}", team, s),
		None => println!("Not found team: {}", team),
	}
}

fn main() {
	// struct alloc in heap
	let p1 = Point { x: 0, y: 0};
	let p2 = &p1;
	let p3 = Point::new(1, 1);
	println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);
	
	println!("******Vec*******");
	let v = vec![1, 2, 3, 4];
	for i in &v {
		println!("{}", *i);
	}
	
	let c1 = String::from("1");
	let c2 = String::from("2");
	let v = vec![c1, c2];
	// -- move occurs because `c1` has type `String`, which does not implement the `Copy` trait
	//println!("c1: {}, c2: {}", c1, c2);
	
	// -- move occurs because value has type `String`, which does not implement the `Copy` trait
	//let v0 = v[0];
	let v0 = &v[0];
	println!("v0: {}, first of v: {}", v0, v[0]);
	
	println!("*******String*******");
	let hello = "你好";
	let whoami = String::from("我是谁");
	// ^^^^^^^^ string indices are ranges of `usize`
	//let h = &hello[0];
	// thread 'main' panicked at 'byte index 2 is not a char boundary;
	//let s = &hello[0..2];
	
	println!("hello: {}, len: {}; whoami: {}, len: {}", 
		hello, hello.len(), whoami, whoami.len());
		
	let say_hello = format!("{}，{}！", hello, whoami);
	println!("{},len: {}", say_hello, say_hello.len());
	
	println!("{}'s chars:", hello);
	for c in hello.chars() {
		println!("{}", c);
	}
	println!("{}'s bytes:", hello);
	for b in hello.bytes() {
		println!("{:#02X}", b);
	}	
	
	println!("*******HashMap*******");
	let mut scores = HashMap::new();
	// &str and String are different type
	//scores.insert("Blue", 10);
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	
	let team1 = String::from("Blue");
	let team2 = String::from("Green");
	find_team_score(&scores, &team1);
	find_team_score(&scores, &team2);
	
	for (key, value) in &scores {
		println!("key: {}, value: {}", key, value);
	}
	
	println!("insert same key");
	scores.insert(String::from("Yellow"), 70);
	println!("{:?}", scores);
	
	println!("insert when key not exist");
	scores.entry(String::from("Green")).or_insert(100);
	scores.entry(String::from("Yellow")).or_insert(50);
	println!("{:?}", scores);
	
	println!("update exist key value");
	let text = "hello world wonderful world";
	let mut word_count = HashMap::new();
	for word in text.split_whitespace() {
		// The or_insert method returns a mutable reference (&mut V) to the value for the specified key. 
		let count = word_count.entry(word).or_insert(0);
		*count += 1;
	}
	println!("{:?}", word_count);
}
