#include "matrix.h"

int get_matrix_data(char *matrix_file_path){
    FILE *p_file;
    p_file = fopen(matrix_file_path, "r");

}

unsigned int **matrix_init(int matrix_dimension) {
    unsigned int **matrix;
    int i;
    matrix = (unsigned int **) malloc(matrix_dimension * sizeof(void *));
    for (i = 0; i < matrix_dimension; ++i)
        matrix[i] = calloc(matrix_dimension, sizeof(unsigned int));
    return matrix;
}

void mm_destroy(int n, double **m) {
    int i;
    for (i = 0; i < n; ++i)
        free(m[i]);
    free(m);
}

double **matrix_gen(int n, double seed) {
    double **m, tmp = seed / n / n;
    int i, j;
    m = matrix_init(n);
    for (i = 0; i < n; ++i)
        for (j = 0; j < n; ++j)
            m[i][j] = tmp * (i - j) * (i + j);
    return m;
}

// better cache performance by transposing the second matrix
double **mm_mul(int n, double *const *a, double *const *b) {
    int i, j, k;
    double **m, **c;
    m = matrix_init(n);
    c = matrix_init(n);
    for (i = 0; i < n; ++i) // transpose
        for (j = 0; j < n; ++j)
            c[i][j] = b[j][i];
    for (i = 0; i < n; ++i) {
        double *p = a[i], *q = m[i];
        for (j = 0; j < n; ++j) {
            double t = 0.0, *r = c[j];
            for (k = 0; k < n; ++k)
                t += p[k] * r[k];
            q[j] = t;
        }
    }
    mm_destroy(n, c);
    return m;
}

double calc(int number) {
    number = number / 2 * 2;
    double **matrix_a = matrix_gen(number, 1.0);
    double **matrix_b = matrix_gen(number, 2.0);
    double **matrix_result = mm_mul(number, matrix_a, matrix_b);
    double result = matrix_result[number / 2][number / 2];
    mm_destroy(number, matrix_a);
    mm_destroy(number, matrix_b);
    mm_destroy(number, matrix_result);
    return result;
}