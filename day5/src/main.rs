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

//Part one
fn find_nbr_vec( input:String ) -> i32 {
	let mut index_line:i32 = 0;
	for line in input.lines() {
		match line.find(char::is_numeric) {
			None => index_line += 1,
			Some(_x) => break,
		};
	}
	let temp:Vec<&str> = input.split("\n").collect();
	let mut result:i32 = 0;
	for i in temp[index_line as usize].chars() {
		if i.is_numeric() {
			result += 1;
		}
	}
	return result;
}

fn  fill_stack( result: &mut Vec<Vec<char>>, input:String, nbr_vec:i32 ) {
	for line_index in (0..nbr_vec).rev() {
		let mut index_vec:usize = 0;
		let mut pos = 1;
		if input.lines().nth(line_index as usize).unwrap()
			.chars().nth(pos as usize).unwrap().is_alphabetic() {
			result[index_vec].push(input.lines().nth(line_index as usize).unwrap()
				.chars().nth(pos as usize).unwrap());
		}

		pos += 4;
		while pos < input.lines().nth(line_index as usize).unwrap().len() && index_vec < result.capacity(){
			index_vec +=1;

			if input.lines().nth(line_index as usize).unwrap()
				.chars().nth(pos as usize).unwrap().is_alphabetic() {
				result[index_vec].push(input.lines().nth(line_index as usize).unwrap()
					.chars().nth(pos as usize).unwrap());
			}

			pos += 4;
		}
	}
}

fn create_crane( input:String ) ->Vec<Vec<char>> {
	let mut result:Vec<Vec<char>> = Vec::new();
	let nbr_vec:i32 = find_nbr_vec(input.clone());
	for _i in 0..nbr_vec {
		result.push(Vec::<char>::new());
	}
	fill_stack( &mut result, input.clone(), nbr_vec);
	return result;
}

fn clean_input( input: &mut String ) -> Vec<&str> {
	let mut line:Vec<&str>;
	line = input.lines().collect();
	line.reverse();
	loop {
		match line[line.len() - 1].chars().nth(0) {
			Some(x) => if x != 'm' { line.pop().unwrap(); },
			None => break ,
		};
	}
	line.pop();
	line.reverse();
	return line;
}

fn clean_txt_instru( txt_instru: &mut Vec<Vec<&str>> ) {
	let nothing_str = "";
	for index in &mut *txt_instru {
		index.retain(|&x| x != nothing_str);
	}
	for index in txt_instru {
		for i in index {
			*i  = i.trim();
		}
	}
}

fn fill_instructions(instruction: &mut Vec<[i32;3]>, input:&mut String ) {
	 let temp = clean_input(input);
	 let mut txt_instru:Vec<Vec<&str>> = Vec::new();
	for i in 0..temp.len() {
		let temp:Vec<&str> = temp[i].split(char::is_alphabetic).collect();
		txt_instru.push(temp);
	}
	clean_txt_instru(&mut txt_instru);

	for line in txt_instru {
		let mut temp:[i32;3] = [0;3];
		temp[0] = line[0].parse::<i32>().unwrap();
		temp[1] = line[1].parse::<i32>().unwrap() - 1;
		temp[2] = line[2].parse::<i32>().unwrap() - 1;
		instruction.push(temp);
	}
}

fn print_result( crane:Vec<Vec<char>> ) {
	for i in crane {
		print!("{}", i[i.len() - 1]);
	}
	println!("");
}

//Part One
fn execute_instruc( instruction:Vec<[i32;3]>, crane:&mut Vec<Vec<char>> ) {
	for order in instruction {
		for _nbr in 0..order[0] {
			let len = crane[order[1] as usize].len();
			if len == 0 {
				break ;
			}
			let temp = crane[order[1] as usize][len - 1];
			crane[order[1] as usize].pop().unwrap();
			crane[order[2] as usize].push(temp);
		}
	}
}

//Part Two
fn  execute_instruc_crate_9001( instruction:Vec<[i32;3]>, crane:&mut Vec<Vec<char>> ) {
	for order in instruction {
		for nbr in (0..order[0]).rev() {
			let len = crane[order[1] as usize].len();
			let c_temp = crane[order[1] as usize][len - ((nbr + 1) as usize)];
			crane[order[2] as usize].push(c_temp);
		}
		for _nbr in 0..order[0] {
			crane[order[1] as usize].pop();
		}
	}
}

fn main() {
	let part = 2;
	let mut input:String = open_file("input");
	let mut crane:Vec<Vec<char>> = create_crane(input.clone());
	let mut instruction:Vec<[i32;3]> = Vec::new();
	fill_instructions(&mut instruction, &mut input);
	if part == 1 {
		execute_instruc(instruction, &mut crane);
	}
	else if part == 2 {
		execute_instruc_crate_9001(instruction, &mut crane);
	}
	print_result(crane);
}
