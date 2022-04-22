use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
	fn new(n: usize) -> Self {
		assert!(n > 0 && n <= 255);
		
		let mut workers = Vec::with_capacity(n);
		let (sender, receiver) = mpsc::channel();
		
		let receiver = Arc::new(Mutex::new(receiver));
		
		for i in 0..n {
			workers.push(Worker::new(i, Arc::clone(&receiver)));
		}
		
		ThreadPool { workers, sender }
	}
	
	fn execute<F>(&self, f: F)
		where F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);
		
		self.sender.send(job).unwrap();
	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {		
		let thread = thread::spawn(move || loop {
			let job = receiver.lock().unwrap().recv().unwrap();
			
			println!("Worker {} got a job; executing.", id);
			
			job();
		});	
		/// This code compiles and runs but doesn’t result in the desired threading behavior: a slow request will still cause other requests to wait to be processed. 
		/// The reason is somewhat subtle: the Mutex struct has no public unlock method because 
		///		the ownership of the lock is based on the lifetime of the MutexGuard<T> within the LockResult<MutexGuard<T>> that the lock method returns.
		/// while let (and if let and match) does not drop temporary values until the end of the associated block.
		/*
		let thread = thread::spawn(move || {
			while let Ok(job) = receiver.lock().unwrap().recv() {
				println!("Worker {} got a job; executing.", id);
			
				job();
			}
		});
		*/
		
		Worker {
			id,
			thread,
		}
	}
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(4);
	
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		
		pool.execute(|| {
			handle_connection(stream);
		});
	}
}

fn handle_connection(mut tcpstream: TcpStream) {
	let mut buf = [0; 1024];
	
	tcpstream.read(&mut buf).unwrap();
	
	//println!("Request: {}", String::from_utf8_lossy(&buf[..]));
	
	let get = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";
	let (headline, filename) = if buf.starts_with(get) {
		("HTTP/1.1 200 OK", "hello.html")
	} else if buf.starts_with(sleep) {
		thread::sleep(std::time::Duration::from_secs(5));
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