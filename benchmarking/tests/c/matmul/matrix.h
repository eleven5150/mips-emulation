#include <math.h>
#include <stdio.h>
#include <stdlib.h>

typedef unsigned long long MatrixItem_t;
typedef MatrixItem_t * MatrixRow_t;
typedef MatrixItem_t ** Matrix_t;


void matrix_print(Matrix_t matrix, unsigned int matrix_dimension);

Matrix_t matrix_init(unsigned int matrix_dimension);

void matrix_free(unsigned int matrix_dimension, Matrix_t matrix);

Matrix_t matrix_multiply(unsigned int matrix_dimension, Matrix_t matrix_a, Matrix_t matrix_b);