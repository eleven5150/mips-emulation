#include <stdio.h>
#include <stdlib.h>

typedef unsigned long long SortItem_t;
typedef SortItem_t * SortData_t;

SortItem_t partition(SortData_t data, long long low, long long high);
void quick_sort(SortData_t data, long long low, long long high);
void data_print(SortData_t data, unsigned int size);