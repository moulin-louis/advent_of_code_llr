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

fn create_crane( input:String ) ->Vec<Vec<char>> {
    let mut result:Vec<Vec<char>> = Vec::new();
    let nbr_vec:i32 = find_nbr_vec(input);
    for pos in (nbr_vec - 1)..0 {
        add_crate_to
    }
    return result;
}

fn main() {
    let input:String = open_file("input_test");
    let mut crane:Vec<Vec<char>> = create_crane(input.clone());
}
