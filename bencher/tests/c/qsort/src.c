#include "src.h"

// Function for partiotion the array into 2 subarrays
int partition(int arr[], int low, int high) 
{
    int temp;
    int pivot = arr[high]; // assuming last element of the array as the pivot element
    int i = (low - 1);     // assuming the index of i pointer as one position less than the first element


    for (int j = low; j <= high - 1; j++) // assuming the index of j pointer as the first position
    { 
        //----If current element is smaller than or equal to pivot----
        if (arr[j] <= pivot) 
        {
            i++;             // increment index of i pointer and swap the elemets at index i and j
            temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        } 
    }


    //----swapping the pivot (last) element and element at i + 1 index----
    temp = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;


    //----returning the index of pivot element having lesser elements to the left and greater elements to the right----
    return (i + 1); 
}

// Recursive function for sorting
void quick_sort(int arr[], int low, int high) 
{ 
    if (low < high) 
    {
        // partitioning the single array into two sub-arrays 
        int pi = partition(arr, low, high); 

        // sorting the sub-arrays
        quick_sort(arr, low, pi - 1); 
        quick_sort(arr, pi + 1, high); 
    } 
}

// Prints sorted array
void print_array(int array[], int n)
{
    for(int i = 0; i < n; i++)
    {
        printf("%d ", array[i]);
    }
}

// Reads data (numbers) from file and pastes it into array
void get_data(char* file_name, int* array, size_t size)
{
    // Opening file for data reading
    FILE *file_ptr = fopen(file_name, "r");

    if (file_ptr == NULL)
    {
        perror("Error while opening file.\n");
        exit(0);
    }
    
    // Reading numbers into array, which will be sorted further
    size_t i = 0;
    while(i < size) {
        if (fscanf(file_ptr, "%d", &array[i]) == EOF) {
            printf("Error while reading from file.\n");
        };
        ++i;
    }

    fclose(file_ptr);
}