#include "matrix.h"
#include "stdio.h"
#include "string.h"
#include "stdlib.h"

#define RECORD_SIZE 12
#define DIMENSION_OFFSET 2

unsigned int MATRIX_DIMENSION = 0;


Matrix_t create_matrix(char *file_path) {
    FILE *fstream = fopen(file_path, "r");
    if (fstream == NULL) {
        printf("file %s opening failed\n", file_path);
        exit(EXIT_FAILURE);
    }

    char header[RECORD_SIZE] = {0};

    fgets(header, RECORD_SIZE, fstream);
    MATRIX_DIMENSION = (unsigned int)strtol(&header[DIMENSION_OFFSET], NULL, 10);
    Matrix_t matrix = matrix_init(MATRIX_DIMENSION);

    unsigned int line_size = MATRIX_DIMENSION * RECORD_SIZE;
    char *buffer = (char *) malloc(line_size * sizeof(char));
    int i = 0, j = 0;
    char *record, *line;

    while ((line = fgets(buffer, line_size, fstream)) != NULL) {
        record = strtok(line, ",");
        while (record != NULL) {
            matrix[i][j] = (MatrixItem_t) strtoull(record, NULL, 10);
            record = strtok(NULL, ",");
            j++;
        }
        ++i;
        j = 0;
    }

    free(buffer);

//    matrix_print(matrix, MATRIX_DIMENSION);
    return matrix;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Error! Both paths to matrix data required\n");
        exit(EXIT_FAILURE);
    }

    char *matrix_a_file = argv[1];
    char *matrix_b_file = argv[2];

    Matrix_t matrix_a = create_matrix(matrix_a_file);
    Matrix_t matrix_b = create_matrix(matrix_b_file);

    Matrix_t matrix_result = matrix_multiply(MATRIX_DIMENSION, matrix_a, matrix_b);

    printf("%llu\t", matrix_result[1][1]);
//    matrix_print(matrix_result, MATRIX_DIMENSION);

    matrix_free(MATRIX_DIMENSION, matrix_a);
    matrix_free(MATRIX_DIMENSION, matrix_b);
    matrix_free(MATRIX_DIMENSION, matrix_result);
}