cimport libc.stdio as c
from libc.stdlib cimport malloc, calloc, free, atoi

import sys

def main(raw_args):
    if len(raw_args) < 1:
        print("Error! File with prime number count must be specified\n")
        exit(1)

    prime_number_count_path: str = raw_args[1]

    prime_number_count_path_encoded = prime_number_count_path.encode("ascii")
    cdef char *prime_number_count_file = <char *>prime_number_count_path_encoded

    cdef c.FILE *fstream = c.fopen(prime_number_count_file, "r")
    if fstream == NULL:
        c.printf("file %s opening failed\n", prime_number_count_file)
        exit(1)

    c.fseek(fstream, 0, c.SEEK_END)
    cdef int file_size = c.ftell(fstream)
    c.fseek(fstream, 0, c.SEEK_SET)

    cdef char *buffer = <char *> malloc(file_size * sizeof(char));
    c.fgets(buffer, file_size, fstream)

    c.fclose(fstream)

    cdef unsigned int prime_number_count = <unsigned int> atoi(buffer)
    free(buffer)

    cdef unsigned int curr_number = 0
    cdef unsigned int j = 0
    while prime_number_count > 0:
        curr_number += 1

        j = 0
        for i in range(1, curr_number + 1):
            # print(f"i -> {i}")
            if curr_number % i == 0:
                j += 1

        if j == 2:
            prime_number_count -= 1

    c.printf("The latest prime number: %d\n", curr_number)


main(sys.argv)