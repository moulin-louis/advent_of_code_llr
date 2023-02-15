use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn open_file() -> String {
	let file = File::open("input").unwrap();
	let mut input = String::new();
	let mut buffer = BufReader::new(file);

	buffer.read_to_string(&mut input).unwrap();
	input
}

fn calculate_score (op_move:char, my_move:char ) -> u32 {
	let mut result:u32;

	result = 0;
	//I use rock
	if my_move == 'A' {
		result += 1;
	}
	//I use paper
	else if my_move == 'B' {
		result += 2;
	}
	//I use scissors
	else {
		result += 3;
	}
	//I lose
	if op_move == 'A' && my_move == 'C' {
		result += 0;
	}
	// I lose
	else if op_move == 'B' && my_move == 'A' {
		result += 0;
	}
	//I lose
	else if op_move == 'C' && my_move == 'B' {
		result += 0;
	}
	//draw
	else if op_move == my_move {
		result += 3;
	}
	//i won
	else {
		result += 6;
	}
	// print!("op_move = {} ", op_move);
	// print!("my_move = {} ", my_move);
	// println!("reslult = {}", result);
	result
}

fn  calcul_pts( input:String, pos:usize ) -> u32 {
	let mut my_move:char = input.chars().nth(pos + 2). unwrap();
	let op_move:char = input.chars().nth(pos ). unwrap();
	//need to draw
	if my_move == 'Y' {
		my_move = op_move;
		return calculate_score(op_move, my_move);
	}
	//need to loose
	if my_move == 'X' {
		if op_move == 'B' || op_move == 'C' {
			my_move = std::char::from_u32((op_move as u32) - 1).unwrap();
			return calculate_score(op_move, my_move);
		}
		else {
			my_move = 'C';
			return calculate_score(op_move, my_move);
		}
	}
	//need to win
	if my_move == 'Z' {
		if op_move == 'A' || op_move == 'B' {
			my_move = std::char::from_u32(op_move as u32 + 1).unwrap();
			return calculate_score(op_move, my_move);
		} else {
			my_move = 'A';
			return calculate_score(op_move, my_move);
		}
	}
	0
}

fn main() {
	let input = open_file();
	let mut result:u32 = 0;
	let mut it:usize = 0;
	while it < input.len() {
	//while it < 4 * 6 {
		result += calcul_pts(input.clone(), it);
		it += 4;
	}
	println!("{}", result);
}
