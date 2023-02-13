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

fn add_value(string:&str) -> i32 {
    let mut result:i32 = 0;

    let split = string.split("\n");
    let split: Vec<&str> = split.collect();
    for i in 0..split.len() {
        let temp = String::from(split[i]);
        result += temp.parse::<i32>().unwrap();
    }
    result
}

fn parse_input(input: Vec<&str>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..input.len() {
        result.push(add_value(input[i]));
    }
    result
}

fn main() {
    let  input = open_file();
    let  input = input.split("\n\n");
    let input: Vec<&str> = input.collect();
    let mut vec_int = parse_input(input);
    vec_int.sort();
    let result = vec_int[vec_int.len() - 1] + vec_int[vec_int.len() - 2] + vec_int[vec_int.len() - 3];
    println!("{}", result);
    return ;
}
