mod blog {
	enum State {
		Draft,
		Review,
		Published,
	}

	pub struct Post {
		text: String,
		state: State,
	}

	impl Post {
		pub fn new() -> Self {
			Post {
				text: String::new(),
				state: State::Draft,
			}
		}
		
		pub fn add_text(&mut self, text: &str) {
			self.text.push_str(text);
		}
		
		pub fn content(&self) -> &str {
			match self.state {
				State::Draft | State::Review => "",
				State::Published => &self.text,
			}
		}
		
		pub fn request_review(&mut self) {
			match self.state {
				State::Draft => self.state = State::Review,
				_ => {},
			}
		}
		
		pub fn approve(&mut self) {
			match self.state {
				State::Review => self.state = State::Published,
				_ => {},
			}
		}
	}
}

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
	println!("draft post content: {}", post.content());

    post.request_review();
    assert_eq!("", post.content());
	println!("review post content: {}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
	println!("published post content: {}", post.content());
}