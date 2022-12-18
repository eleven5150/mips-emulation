use std::fs::File;
use std::io::{BufRead, BufReader};


fn simple_multiply_a_b(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if b.len() == 0 {
        return vec![];
    } else {
        if b[0].len() == 0 {
            return vec![];
        }
    }

    if a.len() == 0 {
        return vec![];
    } else {
        if a[0].len() == 0 {
            return vec![];
        }
    }

    assert_eq!(a.len(), a[0].len());
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), b[0].len());

    let n = b.len();

    let mut c = vec![vec![0_i64; n]; n];

    for j in 0..n {
        for i in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn main() {

    let mut f = BufReader::new(File::open("tests/datasets/matmul/dataset.txt").unwrap());
    let arr: Vec<Vec<i64>> = f.lines().map(|l| l.unwrap().split_whitespace().map(|number| number.parse().unwrap()).collect()).collect();
    let arr2:Vec<Vec<i64>> = arr.clone();
    simple_multiply_a_b(arr,arr2);
}