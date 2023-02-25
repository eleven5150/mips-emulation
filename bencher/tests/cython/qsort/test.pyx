cimport libc.stdio as c

DEF CUTOFF = 17
cdef size_t size = 1000000 
cdef int array[1000000]
cdef c.FILE *file_ptr = c.fopen("tests/datasets/qsort/dataset.txt", "r")
cdef size_t i = 0
while(i < size):
    if (c.fscanf(file_ptr, "%d", &array[i]) == c.EOF):
        c.printf("Error while reading from file.\n")
    i=i+1    
c.fclose(file_ptr)
qsort(array, 0, size)
print(array)

cdef void qsort(int* a, Py_ssize_t start, Py_ssize_t end):
    if (end - start) < CUTOFF:
        insertion_sort(a, start, end)
        return
    cdef Py_ssize_t boundary = partition(a, start, end)
    qsort(a, start, boundary)
    qsort(a, boundary+1, end)

cdef Py_ssize_t partition(int* a, Py_ssize_t start, Py_ssize_t end):
    assert end > start
    cdef Py_ssize_t i = start, j = end-1
    cdef int pivot = a[j]
    while True:
        # assert all(x < pivot for x in a[start:i])
        # assert all(x >= pivot for x in a[j:end])

        while a[i] < pivot:
            i += 1
        while i < j and pivot <= a[j]:
            j -= 1
        if i >= j:
            break
        assert a[j] < pivot <= a[i]
        swap(a, i, j)
        assert a[i] < pivot <= a[j]
    assert i >= j and i < end
    swap(a, i, end-1)
    assert a[i] == pivot
    # assert all(x < pivot for x in a[start:i])
    # assert all(x >= pivot for x in a[i:end])
    return i

cdef inline void swap(int* a, Py_ssize_t i, Py_ssize_t j):
    a[i], a[j] = a[j], a[i]
            
cdef void insertion_sort(int* a, Py_ssize_t start, Py_ssize_t end):
    cdef Py_ssize_t i, j
    cdef int v
    for i in range(start, end):
        #invariant: [start:i) is sorted
        v = a[i]; j = i-1
        while j >= start:
            if a[j] <= v: break
            a[j+1] = a[j]
            j -= 1
        a[j+1] = v            