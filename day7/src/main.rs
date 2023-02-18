use std::fs::File;
use std::io::BufReader;
use std::io::Read;

//open a file and return its content as a String
fn open_file( file_path:&str ) -> String {
	let file:File;
	match File::open(file_path) {
		Err(_e) => panic!("Failed to open file"),
		Ok(x) => file = x,
	};
	let mut buffer = BufReader::new(file);
	let     mut result = String::new();

	buffer.read_to_string(&mut result).unwrap();
	result
}

fn main() {
	let input:String = open_file("input_test");
	println!("input = {}", input);
}
