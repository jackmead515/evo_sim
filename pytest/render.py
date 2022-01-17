from math import sin, cos
from tkinter import *
import time
import os
import zlib
import models_pb2

class Container:

    def __init__(self, cycle):
        self.window = Tk()
        self.canvas = Canvas(self.window, width=1000, height=1000, bg='black')
        self.canvas.pack()
        self.cycle = cycle
        self.step_index = 0


    def update_state(self):

        if self.step_index >= len(self.cycle.steps):
            exit(0)

        self.canvas.delete("all")

        creatures = self.cycle.creatures
        step = self.cycle.steps[self.step_index]
        colors = ['red', 'blue', 'lime']

        for creature_id in creatures.keys():
            creature = creatures.get(creature_id)
            state = step.states.get(creature_id)

            tx, ty, rot = state.translation.x, state.translation.y, state.rotation
            rcos, rsin = cos(rot), sin(rot)
            
            color_index = 0

            for block in creature.bounds.blocks:
                x, y = block.position.x, block.position.y
                w, h = block.width, block.height

                # x1 = x + tx
                # y1 = y + ty
                # x2 = x1 + w
                # y2 = y1 + h
                x1 = (x*rcos - y*rsin) + tx,
                y1 = (x*rsin + y*rcos) + ty,
                x2 = ((x+w)*rcos - (y+h)*rsin) + tx,
                y2 = ((x+w)*rsin + (y+h)*rcos) + ty,

                color = colors[color_index]
                color_index += 1
                if color_index >= len(colors):
                    color_index = 0

                #self.canvas.create_line(x1, y1, x2, y2, fill='lime')
                self.canvas.create_rectangle(x1, y1, x2, y2, fill=color)
            
        self.step_index += 1
        

        #exit()
        #verts = list(flatten(cycle))

        # i = 0
        # while i < len(cycle):
        #     v1, v2, v3, v4 = cycle[i], cycle[i+1], cycle[i+2], cycle[i+3]
        #     #self.canvas.create_rectangle(v1[0],v1[1],v3[0],v3[1], fill='lime')
        #     self.canvas.create_line(v1[0],v1[1],v2[0],v2[1], fill='red')
        #     self.canvas.create_line(v2[0],v2[1],v3[0],v3[1], fill='red')
        #     self.canvas.create_line(v3[0],v3[1],v4[0],v4[1], fill='red')
        #     self.canvas.create_line(v4[0],v4[1],v1[0],v1[1], fill='red')
        #     i += 4


        # self.canvas.create_polygon(
        #     verticies,
        #     fill='lime'
        # )

        # index = 0
        # while index < len(cycle):
        #     if index+1 >= len(cycle):
        #         break
        #     v1, v2 = cycle[index], cycle[index+1]
        #     self.canvas.create_line(v1[0],v1[1],v2[0],v2[1], fill='lime')
        #     index += 2

        # for vertex in cycle:
        #     # fill = 'lime'
        #     # if index%3 == 0:
        #     #     fill = 'blue'
        #     # elif index%2 == 0:
        #     #     fill = 'red'
        #     # index+=1
        #     # self.canvas.create_line(
        #     #     vertex[0] - 2,
        #     #     vertex[1] - 2,
        #     #     vertex[0] + 2,
        #     #     vertex[1] + 2,
        #     # )
        #     self.canvas.create_oval(
        #         vertex[0] - 2,
        #         vertex[1] - 2,
        #         vertex[0] + 2,
        #         vertex[1] + 2,
        #         fill='lime'
        #     )

       #self.cycle_index += 1


    def mainloop(self):
        while True:
            self.window.after(1, self.update_state())
            self.window.update()
            time.sleep(100)


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
            



    