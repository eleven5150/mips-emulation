#include "matrix.h"
#include "stdio.h"
#include "string.h"
#include "stdlib.h"

#define MATRIX_DIMENSION 100



int main(int argc, char *argv[]) {
    if (argc < 1) {
        printf("Filepath required");
        exit(EXIT_FAILURE);
    }

    char *matrix_a_file = argv[1];
    unsigned int **matrix_a = matrix_init(MATRIX_DIMENSION);

    FILE *fstream = fopen(matrix_a_file, "r");
    if (fstream == NULL) {
        printf("file opening failed\n");
        exit(EXIT_FAILURE);
    }

    fseek(fstream, 0, SEEK_END);
    int file_size = ftell(fstream);
    fseek(fstream, 0, SEEK_SET);

    char *buffer = malloc(file_size * sizeof(char));
    int i = 0, j = 0;
    char *record, *line;
    while ((line = fgets(buffer, file_size, fstream)) != NULL) {
        record = strtok(line, ",");
        while (record != NULL) {
            matrix_a[i][j] = (unsigned int) atoi(record);
            record = strtok(NULL, ",");
            j++;
        }
        ++i;
        j = 0;
    }

//    for (i = 0; i < 100; i++) {
//        for (j = 0; j < 100; j++) {
//            printf("%u\t", matrix_a[i][j]);
//        }
//        printf("\n");
//    }
}