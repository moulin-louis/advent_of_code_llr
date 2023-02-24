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

fn check_up( map:& Vec<&str>) -> i32 {
	let mut result:i32 = 0;
	for ver in 0..map[0].len() {
		let mut max_seen = 0;
		max_seen = map[0].chars().nth(0).unwrap() as i32 - '0' as i32;
		for pos in 0..map.len() {
			
		}
	}
	result
}

fn main() {
	let input:String = open_file("input_test");
	let map:Vec<&str> = input.split("\n").collect();
	let mut result:i32 = 0;
	result += check_up(&map);
	println!("input = {:?}", result);
}
