import math
import random

import numpy
import matplotlib.pyplot as plot
from scipy.spatial import ConvexHull

def create_points(point_amount=10, uniform=0.1):
    two_pi = 2*math.pi
    step_size = two_pi / point_amount

    angle = 0
    points = []

    while angle < two_pi:
        y = math.sin(angle) * 5 + random.uniform(-uniform, uniform)
        x = math.cos(angle) * 5 + random.uniform(-uniform, uniform)
        points.append([x, y])
        angle += step_size
    
    return numpy.array(points)


if __name__ == "__main__":

    # points = create_points()

    # alpha = alphashape.optimizealpha(points)

    # alpha_shape = alphashape.alphashape(points, alpha)

    # print(dir(alpha_shape))
    # print(point for point in alpha_shape)

    points = create_points()

    print(points)

    hull = ConvexHull(points)
    plot.scatter(points[:, 0], points[:, 1], c='red')
    for simplex in hull.simplices:
        plot.plot(points[simplex, 0], points[simplex, 1], 'k-')
    plot.show()
        


    