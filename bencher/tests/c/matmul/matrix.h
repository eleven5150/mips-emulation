#include <math.h>
#include <stdio.h>
#include <stdlib.h>

unsigned int **matrix_init(int matrix_dimension);

void mm_destroy(int n, double **m);

double **matrix_gen(int n, double seed);

double **mm_mul(int n, double *const *a, double *const *b);

double calc(int number);