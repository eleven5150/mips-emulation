#include <math.h>
#include <stdio.h>
#include <stdlib.h>

void matrix_print(unsigned int **matrix, int matrix_dimension);

unsigned int **matrix_init(int matrix_dimension);

void matrix_free(int matrix_dimension, unsigned int **matrix);

unsigned int **matrix_multiply(int matrix_dimension, unsigned int **matrix_a, unsigned int **matrix_b);