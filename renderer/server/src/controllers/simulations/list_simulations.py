import os

from flask import Blueprint, current_app, jsonify, request

import constants

mod = Blueprint('list_simulations', __name__)

@mod.route('/simulations', methods=['GET'])
def list_simulations():
    sim_folders = os.listdir(constants.simulation_dir)
    sim_ids = sorted([int(f.split('_')[-1]) for f in sim_folders])

    return jsonify(sim_ids), 200