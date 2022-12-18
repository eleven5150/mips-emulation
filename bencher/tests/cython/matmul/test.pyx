cimport libc.stdio as c

def matrix_multiply(u, v, res):
    for i in range(1000):
        for j in range(1000):
            for k in range(1000):
                res[i][j] += u[i][k] * v[k][j]
    return res

    
cdef c.FILE *file_ptr = c.fopen("tests/datasets/matmul/dataset.txt", "r")
cdef size_t i = 0
cdef size_t j = 0
cdef size_t size = 1000
cdef int array[1000][1000] 
cdef int res[1000][1000]

while(i < size):
    while(j < size):
        c.fscanf(file_ptr, "%d", &array[i][j])
        j+=1
    j = 0
    i+=1
c.fclose(file_ptr)
matrix_multiply(array,array,res)
