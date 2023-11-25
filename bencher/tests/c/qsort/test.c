#include <stdio.h>
#include <stdlib.h>
#include "qsort.h"

#define RECORD_SIZE 12
#define DIMENSION_OFFSET 2

unsigned int DATA_TO_SORT_SIZE;

int main(int argc, char** argv) {
    if (argc < 2) {
        printf("Error! File with data to sort must be specified\n");
        exit(EXIT_FAILURE);
    }

    char *file_path = argv[1];
    FILE *fstream = fopen(file_path, "r");
    if (fstream == NULL) {
        printf("file %s opening failed\n", file_path);
        exit(EXIT_FAILURE);
    }

    char buffer[RECORD_SIZE] = {0};

    fgets(buffer, RECORD_SIZE, fstream);
    DATA_TO_SORT_SIZE = (unsigned int)strtoul(&buffer[DIMENSION_OFFSET], NULL, 10);

    SortData_t data_to_sort = (SortData_t)calloc(DATA_TO_SORT_SIZE, sizeof(SortItem_t));

    int i = 0;
    while ((fgets(buffer, RECORD_SIZE, fstream)) != NULL) {
        data_to_sort[i] = (SortItem_t)strtoull(buffer, NULL, 10);
        i++;
    }

    quick_sort(data_to_sort, 0, DATA_TO_SORT_SIZE - 1);
//    data_print(data_to_sort, DATA_TO_SORT_SIZE);
    fclose(fstream);
    free(data_to_sort);

    return 0;
}
