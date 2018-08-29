extern crate tiny_http;

use tiny_http::{Server, Response};

#[derive(Debug)]
struct Counter {
	key: String,
	counter: u8,
}

impl Counter{
	fn validate(&self) -> bool {
		let len = self.key.len();
		if len > 20 {
			return false;
		}
		return true;
	}
}
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
		
		let mut path = request.url().clone().to_owned();
		let mut path_split = path.split('&');
		let params = path_split.collect::<Vec<&str>>();
		if params[0] == "counter" {
			let mut key = String::from(params[1]);
			let counter = 1;
			let cnt =  Counter{ key, counter };
			let result = cnt.validate();
			println!(" {}", result);
		}
    	let response = Response::from_string("hello world");
    	request.respond(response).unwrap();
	}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate_bad(){
		let key = String::from("skdjghalhsdg;fashghk'sHG'SKAHGAGKHSAKH'GHSDKGHSHA");
		let counter= 1;
        let cnt = Counter{key, counter};
        assert_eq!(cnt.validate(), true);
    }
	#[test]
    fn test_validate_good(){
		let key = String::from("one");
		let counter= 1;
        let cnt = Counter{key, counter};
        assert_eq!(cnt.validate(), true);
    }
}