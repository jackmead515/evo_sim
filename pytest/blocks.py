import numpy
import random

import timeit

def create_blocks(size=11, amount=30):
    matrix = numpy.zeros([size, size])
    mid = int((size - 1) / 2)
    row, col = mid, mid

    matrix[row][col] = 7

    created = 1
    while created < amount:
        nrow, ncol = row, col
        r = random.randint(0, 3)
        if r == 0:
            nrow += 1
        elif r == 1:
            nrow -= 1
        elif r == 2:
            ncol += 1
        elif r == 3:
            ncol -= 1
        
        if nrow < 0 or len(matrix) <= nrow:
            continue
        if ncol < 0 or len(matrix[nrow]) <= ncol:
            continue
        if matrix[nrow][ncol] == 7:
            row, col = nrow, ncol
            continue
            
        matrix[nrow][ncol] = 7
        row, col = nrow, ncol
        created += 1
    
    print(matrix)

        
if __name__ == "__main__":
  create_blocks()