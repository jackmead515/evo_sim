from flask import Flask
from flask import request, jsonify

from controllers.app import serve_app
from controllers.simulations import list_simulations, create_simulation
from controllers.cycles import list_cycles, get_cycle, create_cycle

if __name__ == "__main__":

    app = Flask(__name__)

    app.register_blueprint(serve_app.mod)
    app.register_blueprint(list_simulations.mod)
    app.register_blueprint(create_simulation.mod)
    app.register_blueprint(list_cycles.mod)
    app.register_blueprint(get_cycle.mod)
    app.register_blueprint(create_cycle.mod)

    app.run(host='0.0.0.0', port=80)



