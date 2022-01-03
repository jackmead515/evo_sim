import numpy
import random

char = 1

def get_width(matrix):
    minc, maxc = len(matrix), 0
    for row in range(len(matrix)):
        for col in range(len(matrix[row])):
            if matrix[row][col] == char and col < minc:
                minc = col
            if matrix[row][col] == char and col > maxc:
                maxc = col
    return (maxc - minc) + 1


def get_height(matrix):
    height = 0
    for row in range(len(matrix)):
        for col in range(len(matrix[row])):
            if matrix[row][col] == char:
                height += 1
                break
    return height


def create_blocks(amount=20):
    dims = amount * 2
    if dims % 2 == 0:
        dims += 1

    matrix = numpy.zeros([dims, dims], dtype=numpy.uint8)
    mid = int((dims - 1) / 2)
    row, col = mid, mid

    matrix[row][col] = char

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
        if matrix[nrow][ncol] == char:
            row, col = nrow, ncol
            continue
            
        matrix[nrow][ncol] = char
        row, col = nrow, ncol
        created += 1

    height = get_height(matrix)
    width = get_width(matrix)

    print(f'width: {width}, height: {height}')

    for row in range(len(matrix)):
        print()
        for col in range(len(matrix[row])):
            print(matrix[row][col], end=' ')
    print()

        
if __name__ == "__main__":
  create_blocks()