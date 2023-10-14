#include "qsort.h"

unsigned int partition(unsigned int *data,int low, int high) {
    unsigned int temp;
    unsigned int pivot = data[high];
    unsigned int i = (low - 1);

    for (unsigned int j = low; j <= high - 1; j++) {
        if (data[j] <= pivot) {
            i++;
            temp = data[i];
            data[i] = data[j];
            data[j] = temp;
        } 
    }

    temp = data[i + 1];
    data[i + 1] = data[high];
    data[high] = temp;

    return (i + 1); 
}

void quick_sort(unsigned int *data, int low, int high) {
    if (low < high) {
        unsigned int pi = partition(data, low, high);

        quick_sort(data, low, pi - 1);
        quick_sort(data, pi + 1, high);
    }
}

void data_print(unsigned int *data, unsigned int size) {
    for(int i = 0; i < size; i++) {
        printf("%u ", data[i]);
    }
}