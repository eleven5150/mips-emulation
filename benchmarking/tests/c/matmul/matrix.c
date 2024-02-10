#include "matrix.h"

void matrix_print(Matrix_t matrix, unsigned int matrix_dimension) {
    int i,j;
    for (i = 0; i < matrix_dimension; i++) {
        for (j = 0; j < matrix_dimension; j++) {
            printf("%llu\t", matrix[i][j]);
        }
        printf("\n");
    }
    printf("\n");
}


Matrix_t matrix_init(unsigned int matrix_dimension) {
    Matrix_t matrix;
    int i;
    matrix = (Matrix_t) malloc(matrix_dimension * sizeof(MatrixRow_t));
    for (i = 0; i < matrix_dimension; ++i)
        matrix[i] = (MatrixRow_t) calloc(matrix_dimension, sizeof(MatrixItem_t));
    return matrix;
}

void matrix_free(unsigned int matrix_dimension, Matrix_t matrix) {
    int i;
    for (i = 0; i < matrix_dimension; ++i)
        free(matrix[i]);
    free(matrix);
}

Matrix_t matrix_multiply(unsigned int matrix_dimension, Matrix_t matrix_a, Matrix_t matrix_b) {
    int i, j, k;
    Matrix_t matrix_result, t_matrix_b;
    matrix_result = matrix_init(matrix_dimension);
    t_matrix_b = matrix_init(matrix_dimension);
    for (i = 0; i < matrix_dimension; ++i)
        for (j = 0; j < matrix_dimension; ++j)
            t_matrix_b[i][j] = matrix_b[j][i];

    for (i = 0; i < matrix_dimension; ++i) {
        MatrixRow_t p = matrix_a[i], q = matrix_result[i];
        for (j = 0; j < matrix_dimension; ++j) {
            MatrixItem_t t = 0;
            MatrixRow_t r = t_matrix_b[j];
            for (k = 0; k < matrix_dimension; ++k) {
                t += p[k] * r[k];
            }

            q[j] = t;
        }
    }
    matrix_free(matrix_dimension, t_matrix_b);
    return matrix_result;
}