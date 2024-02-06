'use strict';

const process = require('process');
const fs = require('fs');

let MATRIX_DIMENSION = 0;

function matrix_init(matrix_dimension) {
    const matrix = new Array(matrix_dimension);
    for (let i = 0; i < matrix_dimension; i++)
        matrix[i] = new BigUint64Array(matrix_dimension);
    return matrix;
}

function matrix_transpose(matrix, matrix_dimension) {
    let t_matrix = matrix_init(matrix_dimension);
    for (let i = 0; i < matrix_dimension; i++) {
        for (let j = 0; j < matrix_dimension; j++) {
            t_matrix[i][j] = matrix[j][i];
        }
    }
    return t_matrix;
}

function matrix_multiply(matrix_a, matrix_b, matrix_dimension) {
    let matrix_result = matrix_init(matrix_dimension);
    const t_matrix_b = matrix_transpose(matrix_b, matrix_dimension);
    for (let i = 0; i < matrix_dimension; i++) {
        for (let j = 0; j < matrix_dimension; j++) {
            let sum = BigInt(0);
            for (let k = 0; k < matrix_dimension; k++) {
                sum = sum + matrix_a[i][k] * t_matrix_b[j][k];
            }
            matrix_result[i][j] = sum;
        }
    }
    return matrix_result;
}

function matrix_print(matrix, matrix_dimension) {
    for (let i = 0; i < matrix_dimension; i++) {
        for (let j = 0; j < matrix_dimension; j++) {
            process.stdout.write(`${matrix[i][j]}\t`);
        }
        console.log("")
    }
    console.log("\n")
}

function create_matrix(file_path) {

    let data = fs.readFileSync(file_path, "utf8");
    if (!data) {
        console.error(`File ${file_path} opening failed`);
        return 0;
    }
    let i = 0;
    let j = 0;
    let strings = data.split(/\n/);
    strings.pop();
    let matrix_dimension_string = strings[0];
    MATRIX_DIMENSION = matrix_dimension_string.slice(2);
    let matrix = matrix_init(MATRIX_DIMENSION);
    strings = strings.slice(1)
    for (const line_num in strings) {
        let records = strings[line_num].split(/,/);
        for (const record_num in records) {
            matrix[i][j] = BigInt(parseInt(records[record_num], 10));
            j++;
        }
        i++;
        j = 0;
    }

    // matrix_print(matrix, MATRIX_DIMENSION)
    process.stdout.write(`${matrix[0][0]}\n`);
    return matrix;
}

function main(argc, argv) {
    if (argc < 2) {
        console.error("Error! Both paths to matrix data required\n");
        return 1;
    }

    let matrix_a_file = argv[2];
    let matrix_b_file = argv[3];

    let matrix_a = create_matrix(matrix_a_file);
    let matrix_b = create_matrix(matrix_b_file);

    if (!matrix_a || !matrix_b) {
        return 1;
    }

    let matrix_result = matrix_multiply(matrix_a, matrix_b, MATRIX_DIMENSION);

    // matrix_print(matrix_result, MATRIX_DIMENSION)
    process.stdout.write(`${matrix_result[0][0]}\n`);
}

main(process.argv.length, process.argv);
