
"""
0,0   1,0   2,0   3,0   4,0   5,0
0,1   1,1   2,1   3,1   4,1   5,1
0,2   1,2   2,2   3,2   4,2   5,2
0,3   1,3   2,3   3,3   4,3   5,3
0,4   1,4   2,4   3,4   4,4   5,4


i = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24



y = 0, 0, 1, 1, 0, 1, 2, 2, 2, 0, 1, 2, 3, 3, 3, 3, 0, 1, 2, 3, 4, 4, 4, 4, 4
"""

proper_x = "0, 1, 0, 1, 2, 2, 0, 1, 2, 3, 3, 3, 0, 1, 2, 3, 4, 4, 4, 4"

def x_generator(total):
    lmax = 0
    cmax = 0

    for i in range(total):
        if i == 0:
            yield 0
            yield 1
            lmax = 2
        elif cmax < lmax:
            yield cmax
            cmax += 1
        elif cmax == lmax:
            for _ in range(cmax):
                yield cmax
            lmax = cmax+1
            cmax = 0


def y_generator(total):
    lmax = 0
    cmax = 0

    for i in range(total):
        if i == 0:
            yield 0
            yield 0
            yield 1
            yield 1
            lmax = 2
        elif cmax < lmax:
            yield cmax
            cmax += 1
        elif cmax == lmax:
            for _ in range(cmax+1):
                yield cmax
            lmax = cmax+1
            cmax = 0



def gen_y(prev_y, index):
    pass

if __name__ == "__main__":

    total = 20
    gx = x_generator(total)
    gy = y_generator(total)

    for _ in range(total):
        x, y = next(gx), next(gy)
        print(f'{x},{y}')

       
