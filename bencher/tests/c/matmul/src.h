#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef __clang__
#define COMPILER "clang"
#else
#define COMPILER "gcc"
#endif

double **mm_init(int n);

void mm_destroy(int n, double **m);

double **mm_gen(int n, double seed);

double **mm_mul(int n, double *const *a, double *const *b);

double calc(int n);