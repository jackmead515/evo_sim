from google.protobuf.json_format import MessageToDict
from flask import Blueprint, jsonify, request
import requests

import models_pb2 as models

mod = Blueprint('create_cycle', __name__)

@mod.route('/simulations/<simulation_id>/cycles', methods=['POST'])
def create_cycle(simulation_id):
    
    response = requests.post(f'http://127.0.0.1:8000/simulations/{simulation_id}/cycles')

    if response.status_code == 200:

        cycle = models.Cycle()
        cycle.ParseFromString(response.content)

        return jsonify(MessageToDict(cycle, preserving_proto_field_name=True)), 200

    return "ERROR", response.status_code