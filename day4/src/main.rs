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
	let	mut result = String::new();

	buffer.read_to_string(&mut result).unwrap();
	result
}

fn calculate_pos(section:&mut[i32], x:&str) {
	let arr:Vec<&str> = x.split("-").collect();
	section[0] = arr[0].parse().unwrap();
	section[1] = arr[1].parse().unwrap();
}

//Part One
fn check_full_overlap( input:&str ) -> bool {
	let mut	section_1: [i32; 2] = [0, 2];
	let mut	section_2: [i32; 2] = [0, 2];
	let arr:Vec<&str> = input.split(",").collect();

	calculate_pos(&mut section_1, arr[0]);
	calculate_pos(&mut section_2, arr[1]);
	if section_1[0] <= section_2[0] && section_1[1] >= section_2[1] {
		return true;
	}
	if section_2[0] <= section_1[0] && section_2[1] >= section_1[1] {
		return true;
	}
	false
}



//Part two
fn check_partial_overlap( input:&str ) -> bool {
	let mut	section_1: [i32; 2] = [0, 2];
	let mut	section_2: [i32; 2] = [0, 2];
	let arr:Vec<&str> = input.split(",").collect();

	calculate_pos(&mut section_1, arr[0]);
	calculate_pos(&mut section_2, arr[1]);

	if check_range(section_1) < check_range(section_2) {
		for i in section_2[0]..section_2[1] + 1 {
			if is_in_section(i, section_1) {
				return true;
			}
		}
	}
	else {
		for i in section_1[0]..section_1[1] + 1{
			if is_in_section(i, section_2) {
				return true;
			}
		}
	}
	false
}

fn is_in_section( target:i32, section:[i32;2] ) ->bool {
	for i in section[0]..section[1] + 1 {
		if i == target {
			return true;
		}
	}
	false
}

fn check_range( section:[i32; 2] ) -> i32 {
	let mut result:i32 = 0;

	for _i in section[0]..section[1] + 1{
		result += 1;
	}
	result
}	

fn main() {
	let input:String = open_file("input");
	let mut	result:i32 = 0;

	//Part one
	for line in input.lines() {
		if check_full_overlap(line) {
			result += 1;
		}
	}
	println!("result part one = {}", result);

	//Part two
	result = 0;
	for line in input.lines() {
		if check_partial_overlap(line) {
			result += 1;
		}
	}
	println!("result part two = {}", result);
}
