import os
import zlib

from google.protobuf.json_format import MessageToDict
from flask import Blueprint, jsonify, request

import models_pb2 as models
import constants

mod = Blueprint('get_cycle', __name__)

@mod.route('/simulations/<simulation_id>/cycles/<cycle_id>', methods=['GET'])
def get_cycle(simulation_id, cycle_id):
    
    sim_folder = os.path.join(constants.simulation_dir, f'simulation_{simulation_id}')
    cycle_file = os.path.join(sim_folder, 'cycles', f'cycle_{cycle_id}.zip')

    if os.path.isfile(cycle_file):
        with open(cycle_file, 'rb') as f:
            cycle = models.Cycle()
            cycle.ParseFromString(zlib.decompress(f.read()))
            cycle = MessageToDict(cycle, preserving_proto_field_name=True)

            return jsonify(cycle), 200

    return "NOT FOUND", 404