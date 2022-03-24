from collections import deque
from math import sin, cos
from re import L
from tkinter import *
import time
import os
import zlib
import models_pb2

def rgb_hack(rgb):
    return "#%02x%02x%02x" % rgb 

class Container:

    def __init__(self, cycle):
        self.window = Tk()
        self.canvas = Canvas(self.window, width=1200, height=640, bg='black')
        self.canvas.pack()

        self.play_scale = Scale(self.window, from_=0, to=9999, orient=HORIZONTAL, command=self.on_slider)
        self.play_scale.pack()

        self.play_button = Button(self.window, text='Play', command=self.on_play)
        self.play_button.pack()

        self.pause_button = Button(self.window, text='Pause', command=self.on_pause)
        self.pause_button.pack()

        self.cycle = cycle
        self.step_index = 0
        self.last_rendered_step = 0
        self.is_playing = True
        self.last_render = time.perf_counter()


    def on_play(self):
        self.is_playing = True


    def on_pause(self):
        self.is_playing = False


    def on_slider(self, event):
        self.on_pause()
        self.step_index = self.play_scale.get()
        self.render()

    
    def render(self):
        self.canvas.delete("all")

        creatures = self.cycle.creatures
        step = self.cycle.steps[self.step_index]

        rects = deque()

        for creature_id in creatures.keys():
            creature = creatures.get(creature_id)
            state = step.states.get(creature_id)
            color = [int(c) for c in creature.traits.color]

            tx, ty, rot = state.translation.x, state.translation.y, state.rotation
            rcos, rsin = cos(rot), sin(rot)

            for block in creature.bounds.blocks:
                x, y, s = block.position.x, block.position.y, block.size
                x1, y1 = x*s, y*s
                x2, y2 = x1+s, y1+s

                points = [(x1, y1), (x2, y1), (x2, y2), (x1, y2)]

                for i in range(len(points)):
                    p = points[i]
                    nx = (p[0]*rcos - p[1]*rsin) + tx
                    ny = (p[0]*rsin + p[1]*rcos) + ty
                    points[i] = (nx, ny)

                rects.append((rgb_hack(tuple(color[:3])), points))

        for rect in rects:  
            self.canvas.create_polygon(
                *rect[1],
                fill=rect[0],
                activefill='red',
                outline='gray',
                smooth=0
            )


    def update_state(self):

        if self.step_index >= len(self.cycle.steps):
            self.on_pause()
            return

        if not self.is_playing:
            return

        self.render()

        #print(f'render: {time.perf_counter() - self.last_render}')
        self.last_render = time.perf_counter()
        self.last_rendered_step = self.step_index
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
        if cycle_file == "cycle_0.zip":
            with open(os.path.join(sim_folder, cycle_file), 'rb') as f:
                cycle = models_pb2.Cycle()
                cycle.ParseFromString(zlib.decompress(f.read()))

                container = Container(cycle)
                container.mainloop()
                



    