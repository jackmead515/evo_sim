import os
import zlib

from flask import Blueprint, current_app, jsonify, request
from google.protobuf.json_format import MessageToDict

import models_pb2 as models
import constants

mod = Blueprint('create_simulation', __name__)

@mod.route('/simulations', methods=['POST'])
def create_simulation():

    sim_folders = os.listdir(constants.simulation_dir)
    sim_ids = sorted([int(f.split('_')[-1]) for f in sim_folders])

    simulation_id = 1

    if len(sim_ids) > 0:
        simulation_id = sim_ids[-1] + 1

    simconsts = models.Constants()
    simconsts.world_width = 2000
    simconsts.world_height = 2000
    simconsts.max_steps = 1000
    simconsts.creature_amount = 20
    simconsts.initial_brain_size = 50
    simconsts.max_brain_size = 100
    simconsts.min_brain_size = 10
    simconsts.brain_evolve_chance = 0.5
    simconsts.min_brain_weight_nudge = 0.01
    simconsts.max_brain_weight_nudge = 0.5
    simconsts.brain_input_size = 5
    simconsts.brain_output_size = 5
    simconsts.initial_block_amount = 5
    simconsts.min_block_amount = 2
    simconsts.max_block_amount = 20
    simconsts.block_amount_evolve_chance = 0.05
    simconsts.initial_block_size = 5.0
    simconsts.max_block_size = 10.0
    simconsts.min_block_size = 3.0
    simconsts.block_size_evolve_chance = 0.1
    simconsts.min_block_size_nudge = 0.01
    simconsts.max_block_size_nudge = 0.9
    simconsts.block_arrange_evolve_chance = 0.1

    simulation = models.Simulation(constants=simconsts, simulation_id=simulation_id)
    serialized = zlib.compress(simulation.SerializeToString())
   
    sim_folder = os.path.join(constants.simulation_dir, f'simulation_{simulation_id}')
    cycles_folder = os.path.join(sim_folder, 'cycles')

    os.makedirs(sim_folder, exist_ok=True)
    os.makedirs(cycles_folder, exist_ok=True)

    with open(os.path.join(sim_folder, 'simulation.zip'), 'wb') as f:
        f.write(serialized)

    return jsonify(MessageToDict(simulation, including_default_value_fields=True, preserving_proto_field_name=True)), 200


