from tkinter import *
from ast import literal_eval

def load_cycles():
    cycles = []
    with open('./cycle.txt', 'r') as f:
        lines = f.readlines()
        for line in lines:
            cycles.append(literal_eval(line.strip()))
    return cycles

cycles = load_cycles()

def flatten(something):
    if isinstance(something, (list, tuple, set, range)):
        for sub in something:
            yield from flatten(sub)
    else:
        yield something

class Container:

    def __init__(self):
        self.window = Tk()
        self.canvas = Canvas(self.window, width=800, height=640, bg='black')
        self.canvas.pack()
        self.cycle_index = 0

    def update_state(self):

        if self.cycle_index >= len(cycles):
            exit(0)

        self.canvas.delete("all")

        cycle = cycles[self.cycle_index]

        #verts = list(flatten(cycle))

        i = 0
        while i < len(cycle):
            v1, v2, v3, v4 = cycle[i], cycle[i+1], cycle[i+2], cycle[i+3]
            #self.canvas.create_rectangle(v1[0],v1[1],v3[0],v3[1], fill='lime')
            self.canvas.create_line(v1[0],v1[1],v2[0],v2[1], fill='red')
            self.canvas.create_line(v2[0],v2[1],v3[0],v3[1], fill='red')
            self.canvas.create_line(v3[0],v3[1],v4[0],v4[1], fill='red')
            self.canvas.create_line(v4[0],v4[1],v1[0],v1[1], fill='red')
            i += 4


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

        self.cycle_index += 1

    def mainloop(self):
        while True:
            self.window.update()
            self.window.after(1, self.update_state())


if __name__ == "__main__":
    container = Container()
    container.mainloop()