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

fn check_start_com( input:& &[u8], pos:usize ) -> bool {
	if pos > input.len() - 4 {
		return false;
	}
	let mut arr_char:[u8;4] = [0;4];
	arr_char[0] = input[pos];
	arr_char[1] = input[pos + 1];
	arr_char[2] = input[pos + 2];
	arr_char[3] = input[pos + 3];
	arr_char.sort();
	for i in 0..arr_char.len() {
		if i + 1 == arr_char.len() {
			break ;
		}
		if arr_char[i] == arr_char[i + 1] {
			return false;
		}
	}
	return true;
}

fn check_start_msg( input:& &[u8], pos:usize ) -> bool {
	if pos > input.len() - 14 {
		return false;
	}
	let mut arr_char:[u8;14] = [0;14];
	arr_char[0] = input[pos];
	arr_char[1] = input[pos + 1];
	arr_char[2] = input[pos + 2];
	arr_char[3] = input[pos + 3];
	arr_char[4] = input[pos + 4];
	arr_char[5] = input[pos + 5];
	arr_char[6] = input[pos + 6];
	arr_char[7] = input[pos + 7];
	arr_char[8] = input[pos + 8];
	arr_char[9] = input[pos + 9];
	arr_char[10] = input[pos + 10];
	arr_char[11] = input[pos + 11];
	arr_char[12] = input[pos + 12];
	arr_char[13] = input[pos + 13];
	arr_char.sort();
	for i in 0..arr_char.len() {
		if i + 1 == arr_char.len() {
			break ;
		}
		if arr_char[i] == arr_char[i + 1] {
			return false;
		}
	}
	return true;
}

fn search_patern( input: &[u8] ) {
	let mut pos:usize = 0;
	let result_part_one:i32;
	let mut result_part_two:i32 = 0;
	for _letter in input {
		if check_start_com(&input, pos) {
			for i in 0..input.len() {
				if check_start_msg(&input, i) {
					result_part_two = i as i32;
					break ;
				}
			}
			break ;
		}
		pos += 1;
	}
	result_part_one = (pos + 4) as i32;
	println!("result part one = {}", result_part_one);
	println!("result part two = {}", result_part_two + 14);
}


fn main() {

	//open the file as a slice of u8 char
	let input = open_file("input");
	let input:&[u8] = input.as_bytes();
	//perform the search of the pattern
	search_patern(input);
}
