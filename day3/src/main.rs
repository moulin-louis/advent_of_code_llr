use std::fs::File;
use std::io::BufReader;
use std::io::Read;

//open a file and return its content as a String
fn open_file( file_path:&str ) -> String {
	let file:File;
	match File::open(file_path) {
		Err(_e) => panic!("Failed to open file\n"),
		Ok(x) => file = x,
	};
	let mut buffer = BufReader::new(file);
	let	mut result = String::new();

	buffer.read_to_string(&mut result).unwrap();
	result
}

//calculate the size of the curent line
fn size_len( input: &[u8], pos:usize ) -> usize {
	for i in pos..input.len() {
		if input[i] as char == '\n' {
			return i - pos;
		}
	}
	return input.len() - pos;
}

//calculate what is the priority of the item
fn what_priority( i:usize ) -> i32 {
	//println!("i = {} so {} as a char", i, (i as u8) as char);
	if i >= 97 && i <= 122 {
		return (i - 96) as i32;
	}
	if i >= 65 && i <= 90 {
		return (i - 38) as i32;
	}
	0
}

//Part one
fn calculate_priority( input: &[u8], pos: usize ) -> i32 {

	if input[pos] as char == '\n' {
		return 0;
	}

	let size_len = size_len(input, pos);

	let mut arr_1: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	for i in  pos..(pos + size_len / 2) {
		if input[i] as char == '\n' {
			break ;
		}
		let it = input[i] as usize;
		arr_1[it] = 1;
	}

	let mut arr_2: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	for i in (pos + size_len / 2)..(pos + size_len) {
		if i >= input.len() {
			break ;
		}
		if input[i] as char == '\n' {
			break ;
		}
		let it = input[i] as usize;
		arr_2[it] = 1;
	}

	for i in 0..arr_1.len() {
		arr_1[i] += arr_2[i];
	}
	for i in 0..arr_1.len()
	{
		if arr_1[i] == 2 {
			return what_priority(i);
		}
	}
	0
}

//Part two
fn find_badge( line_1:&str, line_2:&str, line_3:&str) -> i32 {

	let mut arr_1: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	for i in line_1.chars() {
		if i == '\n' {
			break ;
		}
		let it = i as usize;
		arr_1[it] = 1
	}

	let mut arr_2: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	for i in line_2.chars() {
		if i == '\n' {
			break ;
		}
		let it = i as usize;
		arr_2[it] = 1
	}

	let mut arr_3: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	for i in line_3.chars() {
		if i == '\n' {
			break ;
		}
		let it = i as usize;
		arr_3	[it] = 1
	}

	for i in 0..arr_1.len() {
		if arr_2[i] == 1 {
			arr_1[i] += 1;
		}
		if arr_3[i] == 1 {
			arr_1[i] += 1;
		}
	}

	for i in 0..arr_1.len() {
		if arr_1[i] == 3 {
			return what_priority(i);
		}
	}
	0
}


fn main() {
	let mut result:i32 = 0;
	let input = open_file("input");
	let input: &[u8] = input.as_bytes();

	//Part one
	let mut i:usize = 0;
	while i < input.len() {
		result += calculate_priority(input, i);

		i += size_len(input, i);
		if i < input.len() && input[i] as char == '\n' {
			i += 1;
		}
	}
	println!("result of part one = {}", result);

	//Part two
	let input = open_file("input");
	let mut _n:i32 = 0;
	result = 0;
	for _i in input.lines().step_by(3) {
		result += find_badge(input.lines().nth(_n as usize).unwrap(),
							 input.lines().nth((_n + 1) as usize).unwrap(),
							 input.lines().nth((_n + 2 )as usize).unwrap());
		_n += 3;
	}
	println!("result part two = {}", result);
}
