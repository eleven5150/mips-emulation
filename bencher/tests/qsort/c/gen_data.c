#include "gen_data.h"


int main(int argc, char** argv)
{
    int size = atoi(argv[1]);
    printf("%d", size);

    int* dataset = (int*) malloc(sizeof(int) * size);


    srand(time(NULL));

    for (size_t i = 0; i < size; ++i)
    {
        dataset[i] = rand() % (size + 1);
    }

    write_data("dataset.txt", dataset, size);

    free(dataset);

    return 0;
}



void write_data(char* file_name, int* data, size_t size)
{
    FILE* file_ptr = fopen(file_name, "w");

    if(file_ptr == NULL) {
        perror("Error while opening file.\n");
        exit(0);
    }

    for(size_t i = 0; i < size; ++i)
    {
        fprintf(file_ptr, "%d ", data[i]);
    }

    fclose(file_ptr);
}