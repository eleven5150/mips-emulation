use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

static mut DATA_TO_SORT_SIZE: u64 = 0;

fn partition(data: &mut Vec<u64>, low: i64, high: i64) -> i64 {
    let pivot = data[high as usize];
    let mut i: i64 = low - 1;

    for j in low as usize..high as usize {
        if data[j] <= pivot {
            i += 1;
            (data[i as usize], data[j]) = (data[j], data[i as usize]);
        }
    }

    (data[i as usize + 1], data[high as usize]) = (data[high as usize], data[i as usize + 1]);

    return i + 1;
}


fn quick_sort(data_to_sort: &mut Vec<u64>, low: i64, high: i64) {
    if low < high {
        let pi: i64 = partition(data_to_sort, low, high);

        quick_sort(data_to_sort, low, pi - 1);
        quick_sort(data_to_sort, pi, high);
    }
}

fn data_print(data_to_sort: &Vec<u64>) {
    for it in data_to_sort {
        println!("{}", it);
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Error! File with data to sort must be specified\n");
        exit(1);
    }

    let file_path: &String = &args[1];
    let file: File = File::open(file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut first_line: String = String::new();
    let len: usize = reader.read_line(&mut first_line).unwrap();

    unsafe {
        DATA_TO_SORT_SIZE = first_line[2..].trim().parse::<u64>().unwrap();
    }

    let mut data_to_sort: Vec<u64> = Vec::new();
    for line in reader.lines() {
        data_to_sort.push(line.unwrap().trim().parse::<u64>().unwrap());
    }

    unsafe {
        quick_sort(&mut data_to_sort, 0, (DATA_TO_SORT_SIZE - 1) as i64);
    }

//     data_print(&data_to_sort);
}