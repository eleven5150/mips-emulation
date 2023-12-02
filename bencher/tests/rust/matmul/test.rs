use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

static mut MATRIX_DIMENSION: u64 = 0;

fn matrix_multiply(
    matrix_dimension: u64,
    matrix_a: Vec<Vec<u64>>,
    matrix_b: Vec<Vec<u64>>
) -> Vec<Vec<u64>> {
    let mut t_matrix_b: Vec<Vec<u64>> = vec![
        vec![0_u64; matrix_dimension as usize];
        matrix_dimension as usize
    ];

    for i in 0..matrix_dimension as usize {
        for j in 0..matrix_dimension as usize {
            t_matrix_b[i][j] = matrix_b[j][i];
        }
    }

    let mut matrix_result: Vec<Vec<u64>> = vec![
        vec![0_u64; matrix_dimension as usize];
        matrix_dimension as usize
    ];
    let mut sum: u64;
    for i in 0..matrix_dimension as usize {
        for j in 0..matrix_dimension as usize {
            sum = 0;
            for k in 0..matrix_dimension as usize {
                sum += matrix_a[i][k] * t_matrix_b[j][k];
            }
            matrix_result[i][j] = sum;
        }
    }

    return matrix_result;
}

fn print_matrix(matrix: &Vec<Vec<u64>>) {
    for row in matrix {
        for item in row {
            print!("{}\t", item);
        }
        print!("\n");
    }
    print!("\n");
}

fn create_matrix(file_name: &String) -> Vec<Vec<u64>> {
    let file: File = File::open(file_name).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut first_line: String = String::new();
    let len: usize = reader.read_line(&mut first_line).unwrap();

    unsafe {
        MATRIX_DIMENSION = first_line[2..].trim().parse::<u64>().unwrap();
    }

    let mut matrix: Vec<Vec<u64>> = Vec::new();
    for line in reader.lines() {
        let mut row: Vec<u64> = Vec::new();
        for record in line.unwrap().to_string().split(",") {
            row.push(record.trim().parse::<u64>().unwrap());
        }
        matrix.push(row);
    }

    // print_matrix(&matrix);
    return matrix;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error! Both paths to matrix data required\n");
        exit(1);
    }

    let matrix_a_file: &String = &args[1];
    let matrix_b_file: &String = &args[2];

    let matrix_a: Vec<Vec<u64>> = create_matrix(matrix_a_file);
    let matrix_b: Vec<Vec<u64>> = create_matrix(matrix_b_file);

    unsafe {
       let matrix_result: Vec<Vec<u64>> = matrix_multiply(MATRIX_DIMENSION, matrix_a, matrix_b);
        print_matrix(&matrix_result);
    }
}
