#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
    if (argc < 2) {
        printf("Error! File with prime curr_number count must be specified\n");
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

    unsigned int prime_number_count = (unsigned int)atoi(buffer);
    free(buffer);

    unsigned int curr_number = 0;
    while (prime_number_count > 0) {
        curr_number++;

        unsigned int j = 0;
        for (unsigned int i = 1; i <= curr_number; i++) {
            if (curr_number % i == 0) {
                j++;
            }
        }

        if (j == 2) {
            prime_number_count--;
        }
    }

    printf("The latest prime number: %d\n", curr_number);
    return 0;
}
