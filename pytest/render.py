from collections import deque
from math import sin, cos
from tkinter import *
import time
import os
import zlib
import models_pb2

class Container:

    def __init__(self, cycle):
        self.window = Tk()
        self.canvas = Canvas(self.window, width=800, height=640, bg='black')
        self.canvas.pack()
        self.cycle = cycle
        self.step_index = 0
        self.last_render = time.perf_counter()


    def update_state(self):

        if self.step_index >= len(self.cycle.steps):
            exit(0)

        self.canvas.delete("all")

        creatures = self.cycle.creatures
        step = self.cycle.steps[self.step_index]
        #colors = ['red', 'blue', 'lime']

        rects = deque()

        for creature_id in creatures.keys():
            creature = creatures.get(creature_id)
            state = step.states.get(creature_id)

            tx, ty, rot = state.translation.x, state.translation.y, state.rotation
            rcos, rsin = cos(rot), sin(rot)

            for block in creature.bounds.blocks:
                x, y = block.position.x, block.position.y
                w, h = block.width, block.height
                x, y = x*w, y*h
                
                x1 = (x*rcos - y*rsin) + tx
                y1 = (x*rsin + y*rcos) + ty
                #x2 = (xp*rcos - yp*rsin) + tx
                #y2 = (xp*rsin + yp*rcos) + ty

                rects.append([
                    (x1, y1),
                    (x1+w, y1),
                    (x1+w, y1+h),
                    (x1, y1+h)
                ])

        for rect in rects:  
            #self.canvas.create_line(rect[0][0], rect[0][1], rect[1][0], rect[1][1], fill='lime')
            #self.canvas.create_line(rect[1][0], rect[1][1], rect[2][0], rect[2][1], fill='lime')
            #self.canvas.create_line(rect[2][0], rect[2][1], rect[3][0], rect[3][1], fill='lime')
            #self.canvas.create_line(rect[3][0], rect[3][1], rect[0][0], rect[0][1], fill='lime')
            self.canvas.create_polygon(*rect,
                fill='lime',
                outline='red',
                activefill='red',
                smooth=1
            )

        #print(f'render: {time.perf_counter() - self.last_render}')
        self.last_render = time.perf_counter()
        self.step_index += 1


    def mainloop(self):
        while True:
            self.window.after(1, self.update_state())
            self.window.update()
            #print(f'step: {self.step_index}')
            #time.sleep(100)


if __name__ == "__main__":
    """
    Read simulation cycles and get file names
    """

    simulation_id = 1

    sim_folder = f'simulations/sim_{simulation_id}'
    cycle_files = os.listdir(sim_folder)

    for cycle_file in cycle_files:
        with open(os.path.join(sim_folder, cycle_file), 'rb') as f:
            cycle = models_pb2.Cycle()
            cycle.ParseFromString(zlib.decompress(f.read()))

            container = Container(cycle)
            container.mainloop()
            



    