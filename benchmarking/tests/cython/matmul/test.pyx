cimport libc.stdio as c
from libc.stdlib cimport malloc, calloc, free, strtoull, strtoul
from libc.string cimport strtok, memset

import sys

RECORD_SIZE = 12
MATRIX_DIMENSION = 0
DIMENSION_OFFSET = 2

ctypedef unsigned long long MatrixItem_t
ctypedef MatrixItem_t * MatrixRow_t
ctypedef MatrixItem_t ** Matrix_t


cdef void matrix_print(Matrix_t matrix, unsigned int matrix_dimension):
    for i in range(0, matrix_dimension):
        for j in range(0, matrix_dimension):
            c.printf("%llu\t", matrix[i][j])
        c.printf("\n")
    c.printf("\n")


cdef Matrix_t matrix_init(unsigned int matrix_dimension):
    cdef Matrix_t matrix = <Matrix_t> malloc(matrix_dimension * sizeof(MatrixRow_t))
    for i in range(0, matrix_dimension):
        matrix[i] = <MatrixRow_t> calloc(matrix_dimension, sizeof(MatrixItem_t))
    return matrix


cdef void matrix_free(unsigned int matrix_dimension, Matrix_t matrix):
    for i in range(0, matrix_dimension):
        free(matrix[i])
    free(matrix)


cdef Matrix_t create_matrix(char *file_name):
    global MATRIX_DIMENSION

    cdef c.FILE *fstream = c.fopen(file_name, "r")
    if fstream == NULL:
        c.printf("file %s opening failed\n", file_name)
        exit(1)

    cdef char *header = <char *> calloc(<int> RECORD_SIZE, sizeof(char))

    c.fgets(header, <int> RECORD_SIZE, fstream)
    MATRIX_DIMENSION = <unsigned int> strtoul(&header[<int> DIMENSION_OFFSET], NULL, 10)
    cdef Matrix_t matrix = matrix_init(<MatrixItem_t> MATRIX_DIMENSION)

    cdef unsigned int line_size = <unsigned int> (MATRIX_DIMENSION * RECORD_SIZE)
    cdef char *buffer = <char *> malloc(line_size * sizeof(char))

    cdef char *record
    cdef int i = 0
    cdef int j = 0
    while c.fgets(buffer, line_size, fstream) != NULL:
        record = strtok(buffer, ",")
        while record != NULL:
            matrix[i][j] = <MatrixItem_t> strtoull(record, NULL, 10)
            record = strtok(NULL, ",")
            j += 1
        i += 1
        j = 0

    # matrix_print(matrix, <unsigned int> MATRIX_DIMENSION)
    c.fclose(fstream)
    free(header)
    free(buffer)
    return matrix


cdef Matrix_t matrix_multiply(unsigned int matrix_dimension, Matrix_t matrix_a, Matrix_t matrix_b):
    cdef Matrix_t matrix_result
    cdef Matrix_t t_matrix_b
    matrix_result = matrix_init(matrix_dimension)
    t_matrix_b = matrix_init(matrix_dimension)
    for i in range(0, matrix_dimension):
        for j in range(0, matrix_dimension):
            t_matrix_b[i][j] = matrix_b[j][i]

    cdef MatrixRow_t p
    cdef MatrixRow_t q
    cdef MatrixRow_t r
    cdef MatrixItem_t t
    for i in range(0, matrix_dimension):
        p = matrix_a[i]
        q = matrix_result[i]
        for j in range(0, matrix_dimension):
            t = 0
            r = t_matrix_b[j]
            for k in range(0, matrix_dimension):
                t += p[k] * r[k]
            q[j] = t

    matrix_free(matrix_dimension, t_matrix_b)
    return matrix_result


def main(raw_args):
    if len(raw_args) < 2:
        print("Error! Both paths to matrix data required\n")
        exit(1)

    matrix_a_path: str = raw_args[1]
    matrix_b_path: str = raw_args[2]

    matrix_a_path_encoded = matrix_a_path.encode("ascii")
    cdef char *matrix_a_file = <char *>matrix_a_path_encoded

    matrix_b_path_encoded = matrix_b_path.encode("ascii")
    cdef char *matrix_b_file = <char *>matrix_b_path_encoded

    cdef Matrix_t matrix_a = create_matrix(matrix_a_file)
    cdef Matrix_t matrix_b = create_matrix(matrix_b_file)

    cdef Matrix_t matrix_result = matrix_multiply(<int> MATRIX_DIMENSION, matrix_a, matrix_b)

    # matrix_print(matrix_result, <unsigned int> MATRIX_DIMENSION)

    matrix_free(MATRIX_DIMENSION, matrix_a)
    matrix_free(MATRIX_DIMENSION, matrix_b)
    matrix_free(MATRIX_DIMENSION, matrix_result)


main(sys.argv)
