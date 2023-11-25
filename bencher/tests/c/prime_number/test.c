#include <stdio.h>
#include <stdlib.h>

typedef unsigned long long PrimeNumberItem_t;

int main(int argc, char **argv)
{
    if (argc < 2) {
        printf("Error! File with prime number count must be specified\n");
        exit(EXIT_FAILURE);
    }

    char *file_path = argv[1];
    FILE *fstream = fopen(file_path, "r");
    if (fstream == NULL) {
        printf("file %s opening failed\n", file_path);
        exit(EXIT_FAILURE);
    }

    fseek(fstream, 0, SEEK_END);
    int file_size = ftell(fstream);
    fseek(fstream, 0, SEEK_SET);

    char *buffer = (char *) malloc(file_size * sizeof(char));
    fgets(buffer, file_size, fstream);

    fclose(fstream);

    PrimeNumberItem_t prime_number_count = (PrimeNumberItem_t)strtoll(buffer, NULL, 10);
    free(buffer);

    PrimeNumberItem_t curr_number = 0;
    while (prime_number_count > 0) {
        curr_number++;

        PrimeNumberItem_t j = 0;
        for (PrimeNumberItem_t i = 1; i <= curr_number; i++) {
            if (curr_number % i == 0) {
                j++;
            }
        }

        if (j == 2) {
            prime_number_count--;
        }
    }

    printf("The latest prime number: %llu\n", curr_number);
    return 0;
}
