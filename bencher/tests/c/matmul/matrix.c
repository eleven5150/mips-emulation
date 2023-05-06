#include "matrix.h"

void matrix_print(unsigned int **matrix, int matrix_dimension) {
    int i,j;
    for (i = 0; i < matrix_dimension; i++) {
        for (j = 0; j < matrix_dimension; j++) {
            printf("%u\t", matrix[i][j]);
        }
        printf("\n");
    }
    printf("\n");
}


unsigned int **matrix_init(int matrix_dimension) {
    unsigned int **matrix;
    int i;
    matrix = (unsigned int **) malloc(matrix_dimension * sizeof(unsigned int *));
    for (i = 0; i < matrix_dimension; ++i)
        matrix[i] = (unsigned int *) calloc(matrix_dimension, sizeof(unsigned int));
    return matrix;
}

void matrix_free(int matrix_dimension, unsigned int **matrix) {
    int i;
    for (i = 0; i < matrix_dimension; ++i)
        free(matrix[i]);
    free(matrix);
}

unsigned int **matrix_multiply(int matrix_dimension, unsigned int **matrix_a, unsigned int **matrix_b) {
    int i, j, k;
    unsigned int **matrix_result, **t_matrix_b;
    matrix_result = matrix_init(matrix_dimension);
    t_matrix_b = matrix_init(matrix_dimension);
    for (i = 0; i < matrix_dimension; ++i)
        for (j = 0; j < matrix_dimension; ++j)
            t_matrix_b[i][j] = matrix_b[j][i];
    for (i = 0; i < matrix_dimension; ++i) {
        unsigned int *p = matrix_a[i], *q = matrix_result[i];
        for (j = 0; j < matrix_dimension; ++j) {
            unsigned int t = 0, *r = t_matrix_b[j];
            for (k = 0; k < matrix_dimension; ++k)
                t += p[k] * r[k];
            q[j] = t;
        }
    }
    matrix_free(matrix_dimension, t_matrix_b);
    return matrix_result;
}