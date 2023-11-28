#include "qsort.h"

long long partition(SortData_t data, long long low, long long high) {
    SortItem_t temp;
    SortItem_t pivot = data[high];
    long long i = (low - 1);

    for (SortItem_t j = low; j <= high - 1; j++) {
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

void quick_sort(SortData_t data, long long low, long long high) {
    if (low < high) {
        long long pi = partition(data, low, high);

        quick_sort(data, low, pi - 1);
        quick_sort(data, pi + 1, high);
    }
}

void data_print(SortData_t data, unsigned int size) {
    for(int i = 0; i < size; i++) {
        printf("%llu\n", data[i]);
    }
}