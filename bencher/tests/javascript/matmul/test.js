'use strict';

const process = require('process');
const fs = require('fs');

const MATRIX_DIMENSION = 1000;

const Matrix = {
    init: function (matrix_dimension) {
        const matrix = new Array(matrix_dimension);
        for (let i = 0; i < matrix_dimension; i++)
            matrix[i] = new Uint32Array(matrix_dimension);
        return matrix;
    },
    transpose: function (matrix, matrix_dimension) {
        const t_matrix = Matrix.init(matrix_dimension);
        for (let i = 0; i < matrix_dimension; i++) {
            for (let j = 0; j < matrix_dimension; j++) {
                t_matrix[i][j] = matrix[j][i];
            }
        }
        return t_matrix;
    },
    multiply: function (matrix_a, matrix_b, matrix_dimension) {
        const matrix_result = Matrix.init(matrix_dimension);
        const t_matrix_b = Matrix.transpose(matrix_b, matrix_dimension);
        for (let i = 0; i < matrix_dimension; i++) {
            for (let j = 0; j < matrix_dimension; j++) {
                let sum = 0;
                for (let k = 0; k < matrix_dimension; k++)
                    sum = sum + matrix_a[i][k] * t_matrix_b[j][k];
                matrix_result[i][j] = sum;
            }
        }
        return matrix_result;
    },
    print: function (matrix, matrix_dimension) {
        for (let i = 0; i < matrix_dimension; i++) {
            for (let j = 0; j < matrix_dimension; j++) {
                process.stdout.write(`${matrix[i][j]}\t`);
            }
            console.log("\n")
        }
        console.log("\n")
    }
};

function create_matrix(file_path) {
    let matrix = Matrix.init(MATRIX_DIMENSION);
    let data = fs.readFileSync(file_path, "utf8");
    if (!data) {
        console.error(`File ${file_path} opening failed`);
        return 0;
    }
    let i = 0;
    let j = 0;
    let strings = data.split(/\n/);
    strings.pop()
    for (const line_num in strings) {
        let records = strings[line_num].split(/,/);
        for (const record_num in records) {
            matrix[i][j] = parseInt(records[record_num], 10);
            j++;
        }
        i++;
        j = 0;
    }

    // Matrix.print(matrix, MATRIX_DIMENSION)
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

    let matrix_result = Matrix.multiply(matrix_a, matrix_b, MATRIX_DIMENSION);

    // Matrix.print(matrix_result, MATRIX_DIMENSION)
}

main(process.argv.length, process.argv);
