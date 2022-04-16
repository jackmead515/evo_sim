import os

from flask import Blueprint, current_app, jsonify, request

import constants

mod = Blueprint('list_cycles', __name__)

@mod.route('/simulations/<simulation_id>/cycles', methods=['GET'])
def list_cycles(simulation_id):
    
    cycles_folder = os.path.join(constants.simulation_dir, f'simulation_{simulation_id}', 'cycles')

    if os.path.isdir(cycles_folder):
        cycle_ids = sorted([
            int(f.split('_')[-1].split('.')[0]) 
            for f in os.listdir(cycles_folder)
        ])
        return jsonify(cycle_ids), 200

    return "NOT FOUND", 404