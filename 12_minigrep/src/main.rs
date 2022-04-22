use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|error| {
		eprintln!("Problem parsing arguments: {}", error);
		process::exit(1);
	});

    //println!("query string: {}", config.query);
    //println!("search file: {}", config.filename);
	
	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}
