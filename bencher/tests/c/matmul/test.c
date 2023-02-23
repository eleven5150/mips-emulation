#include "matrix.h"
#include "stdio.h"
#include "string.h"
#include "stdlib.h"

#define MATRIX_DIMENSION 1000

unsigned int **create_matrix(char *file_path) {
    unsigned int **matrix = matrix_init(MATRIX_DIMENSION);

    FILE *fstream = fopen(file_path, "r");
    if (fstream == NULL) {
        printf("file %s opening failed\n", file_path);
        exit(EXIT_FAILURE);
    }

    fseek(fstream, 0, SEEK_END);
    int file_size = ftell(fstream);
    fseek(fstream, 0, SEEK_SET);

    char *buffer = malloc(file_size * sizeof(char));
    int i = 0, j = 0;
    char *record, *line;
    while ((line = fgets(buffer, file_size, fstream)) != NULL) {
        record = strtok(line, ",");
        while (record != NULL) {
            matrix[i][j] = (unsigned int) atoi(record);
            record = strtok(NULL, ",");
            j++;
        }
        ++i;
        j = 0;
    }

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

    unsigned int **matrix_a = create_matrix(matrix_a_file);
    unsigned int **matrix_b = create_matrix(matrix_b_file);

    unsigned int **matrix_result = matrix_multiply(MATRIX_DIMENSION, matrix_a, matrix_b);

//    matrix_print(matrix_result, MATRIX_DIMENSION);

    matrix_free(MATRIX_DIMENSION, matrix_a);
    matrix_free(MATRIX_DIMENSION, matrix_b);
    matrix_free(MATRIX_DIMENSION, matrix_result);
}