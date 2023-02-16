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
	let mut input = String::new();
	let mut buffer = BufReader::new(file);

	buffer.read_to_string(&mut input).unwrap();
	input
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

//calculate the size of the next 3 lines
fn len_3_line( input: &[u8], pos_1:usize ) -> usize {
	let mut nbr_line:i32 = 0;

	let mut pos = pos_1;
	if input[pos] as char == '\n' {
		pos += 1;
	}
	for i in pos..input.len() {
		if input[i] as char == '\n' {
			nbr_line += 1;
		}
		if nbr_line == 3 {
			return  i;
		}
	}
	println!("cant find 3 line");
	input.len() - pos
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
fn fing_badge( input: &[u8], pos_1:usize) -> i32 {
	let mut pos:usize = pos_1;
	if input[pos] as char == '\n' {
		pos += 1;
	}
	let mut sz_len = size_len(input, pos);

	//println!("pos = {}", pos);
	let mut arr_1: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	for i in pos..(pos + sz_len) {
		if input[i] as char == '\n' {
			break ;
		}
		let it = input[i] as usize;
		arr_1[it] = 1;
	}

	let mut arr_2: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	pos += sz_len + 1;
	//println!("pos = {}", pos);
	sz_len = size_len(input, pos);
	for i in pos..(pos + sz_len) {
		if input[i] as char == '\n' {
			break ;
		}
		let it = input[i] as usize;
		arr_2[it] = 1;
	}

	let mut arr_3: [u8; u8::MAX as usize] = [0; u8::MAX as usize];
	pos += sz_len + 1;
	//println!("pos = {}", pos);
	sz_len = size_len(input, pos);
	for i in pos..(pos + sz_len) {
		if input[i] as char == '\n' {
			break ;
		}
		let it = input[i] as usize;
		arr_3[it] = 1;
	}

	// println!("{:?}\n", arr_1);
	// println!("{:?}\n", arr_2);
	// println!("{:?}\n", arr_3);

	for i in 0..arr_1.len() {
		if arr_2[i] == 1 {
			arr_1[i] += 1;
		}
		if arr_3[i] == 1 {
			arr_1[i] += 1;
		}
	}
	//println!("{:?}", arr_1);
	for i in 0..arr_1.len() {
		if arr_1[i] == 3 {
			//println!("found a badge");
			return what_priority(i);
		}
	}
	0
}

fn main() {
	let mut result:i32 = 0;
	let input = open_file("input");
	let input: &[u8] = input.as_bytes();
	let mut  i = 0;

	//Part one
	while i < input.len() {
		result += calculate_priority(input, i);

		i += size_len(input, i);
		if i < input.len() && input[i] as char == '\n' {
			i += 1;
		}
	}
	println!("result of part one = {}", result);
	//Part two
	result = 0;
	i = 0;
	let mut nbr_call = 0;
	while i < input.len() {
		nbr_call += 1;
		result += fing_badge(input, i);
		i += len_3_line(input, i);
		println!("i = {}", i);
	}
	println!("nbr call = {}", nbr_call);
	println!("result of part two = {}", result);
}
