use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Error! ile with prime number count must be specified\n");
        exit(1);
    }

    let file_path: &String = &args[1];

    let file: File = File::open(file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut first_line: String = String::new();
    let len: usize = reader.read_line(&mut first_line).unwrap();

    let mut prime_number_count: u64 = first_line.trim().parse::<u64>().unwrap();

    let mut curr_number: u64 = 0;
    while prime_number_count > 0 {
        curr_number += 1;

        let mut j: u64 = 0;
        for i in 1..curr_number + 1 {
            if curr_number % i == 0 {
                j += 1;
            }
        }

        if j == 2 {
            prime_number_count -= 1;
        }
    }

    println!("The latest prime number: {}", curr_number);
}
