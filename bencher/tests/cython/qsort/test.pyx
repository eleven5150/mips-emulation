cimport libc.stdio as c
from libc.stdlib cimport malloc, calloc, free, atoi

import sys

RECORD_SIZE = 12
DIMENSION_OFFSET = 2
DATA_TO_SORT_SIZE = 0

cdef unsigned int partition(unsigned int *data, int low, int high):
    cdef unsigned int temp
    cdef unsigned int pivot = data[high]
    cdef unsigned int i = low - 1

    for j in range(low, high):
        if data[j] <= pivot:
            i += 1
            temp = data[i]
            data[i] = data[j]
            data[j] = temp

    temp = data[i + 1]
    data[i + 1] = data[high]
    data[high] = temp

    return i + 1


cdef void quick_sort(unsigned int *data, int low, int high):
    cdef unsigned int pi = 0
    if low < high:
        pi = partition(data, low, high)

        quick_sort(data, low, pi - 1)
        quick_sort(data, pi + 1, high)


cdef void data_print(unsigned int *data, unsigned int size):
    for i in range(0, size):
        c.printf("%u ", data[i])


def main(raw_args):
    global DATA_TO_SORT_SIZE

    if len(raw_args) < 1:
        print("Error! File with data to sort must be specified\n")
        exit(1)

    data_to_sort_path: str = raw_args[1]

    data_to_sort_path_encoded = data_to_sort_path.encode("ascii")
    cdef char *data_to_sort_file = <char *>data_to_sort_path_encoded

    cdef c.FILE *fstream = c.fopen(data_to_sort_file, "r")
    if fstream == NULL:
        c.printf("file %s opening failed\n", data_to_sort_file)
        exit(1)

    cdef char *buffer = <char *> calloc(<int> RECORD_SIZE, sizeof(char))

    c.fgets(buffer, <int> RECORD_SIZE, fstream)
    DATA_TO_SORT_SIZE = <unsigned int> atoi(&buffer[<int> DIMENSION_OFFSET])

    cdef unsigned int *data_to_sort = <unsigned int *>calloc(DATA_TO_SORT_SIZE, sizeof(unsigned int));

    cdef unsigned int i = 0
    while (c.fgets(buffer, RECORD_SIZE, fstream)) != NULL:
        data_to_sort[i] = <unsigned int>atoi(buffer)
        i += 1

    quick_sort(data_to_sort, 0, DATA_TO_SORT_SIZE - 1)
    # data_print(data_to_sort, DATA_TO_SORT_SIZE)
    c.fclose(fstream)
    free(data_to_sort)


main(sys.argv)