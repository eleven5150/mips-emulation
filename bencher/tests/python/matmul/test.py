import numpy as np

def matmul(X, Y,): 
    length=len(X[0])
    result_matrix = [[0 for i in range(length)] for i in range(length)]
    for i in range(length):
        for j in range(length):
            for k in range(length):
                result_matrix[i][j] += X[i][k] * Y[k][j]
    return result_matrix


size = 1000

mat_one = []
with open("tests/datasets/matmul/dataset.txt") as f:
    for line in f:
       mat_one.append([int(x) for x in line.split()])

mat_two = mat_one
matmul(mat_one,mat_two)




