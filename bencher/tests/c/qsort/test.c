#include "src.h"

int main(int argc, char** argv)
{
    int data_size = atoi(argv[1]);  // User writes the size of array to sort as 1-st argument in terminal
    int dataset[data_size];


    // Understanding current directory for debug
    /*char cwd[1000];
    if (getcwd(cwd, sizeof(cwd)) != NULL) {
       printf("Current working dir: %s\n", cwd);
    } else {
       perror("getcwd() error");
       return 1;
    }*/

    char* file_name = "app/tests/c/qsort/dataset.txt";

    get_data(file_name, dataset, data_size);  // Reading data from file, then copying it in array

    quick_sort(dataset, 0, data_size - 1);  // Sorting the array

    //print_array(dataset, data_size);

    return 0;
}
