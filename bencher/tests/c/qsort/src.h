#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>


int partition(int arr[], int low, int high);
void quick_sort(int arr[], int low, int high);
void print_array(int array[], int n);
void get_data(char* file_name, int* array, size_t size);