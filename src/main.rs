extern crate tiny_http;

use tiny_http::{Server, Response};

fn main() {
	let server = Server::http("0.0.0.0:8000").unwrap();
        println!("tiny http app listening on 0.0.0.0:8000");

	for request in server.incoming_requests() {
    		println!("received request! \nmethod: {:?},\nurl: {:?},\nheaders: ",
        		request.method(),
        		request.url(),
    		);
		for header in request.headers() {
                    println!(" {}", header);
		}
		

    		let response = Response::from_string("hello world");
    		request.respond(response).unwrap();
	}
}
