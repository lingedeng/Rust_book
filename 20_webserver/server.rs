use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		
		handle_connection(stream);
	}
}

fn handle_connection(mut tcpstream: TcpStream) {
	let mut buf = [0; 1024];
	
	tcpstream.read(&mut buf).unwrap();
	
	//println!("Request: {}", String::from_utf8_lossy(&buf[..]));
	
	let get = b"GET / HTTP/1.1\r\n";
	let (headline, filename) = if buf.starts_with(get) {
		("HTTP/1.1 200 OK", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND", "404.html")
	};
	
	let page = fs::read_to_string(filename).unwrap();
	let rsp = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		headline, page.len(), page
	);
		
	tcpstream.write(rsp.as_bytes()).unwrap();
	tcpstream.flush().unwrap();	
}