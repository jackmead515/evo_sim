import os
import zlib

from google.protobuf.json_format import MessageToDict

import renderer.server.src.models_pb2 as models

if __name__ == "__main__":

    cycle_file = './simulations/simulation_1/cycles/cycle_0.zip'

    with open(cycle_file, 'rb') as f:
        cycle = models.Cycle()
        cycle.ParseFromString(zlib.decompress(f.read()))

        print(MessageToDict(cycle))