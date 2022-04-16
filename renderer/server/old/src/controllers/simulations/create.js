const path = require('path');
const PBF = require('pbf');
const fs = require('fs');
const zlib = require('zlib');

const protobuf = require('../../protobuf');

const simsPath = path.join(__dirname, '../../../../../simulations');

const defaultConstants = {
    'world_width': 2000,
    'world_height': 2000,
    'max_steps': 1000,
    'creature_amount': 20,
    'initial_brain_size': 50,
    'max_brain_size': 100,
    'min_brain_size': 10,
    'brain_evolve_chance': 0.5,
    'min_brain_weight_nudge': 0.01,
    'max_brain_weight_nudge': 0.5,
    'brain_input_size': 5,
    'brain_output_size': 5,
    'initial_block_amount': 5,
    'min_block_amount': 2,
    'max_block_amount': 20,
    'block_amount_evolve_chance': 0.05,
    'initial_block_size': 5.0,
    'max_block_size': 10.0,
    'min_block_size': 3.0,
    'block_size_evolve_chance': 0.1,
    'min_block_size_nudge': 0.01,
    'max_block_size_nudge': 0.9,
    'block_arrange_evolve_chance': 0.1
}

function createSimulation(req, res) {
    let constants = req.body;

    const simulationIds = fs.readdirSync(simsPath)
        .filter((file) => file.startsWith('simulation_'))
        .map((file) => parseInt(file.split('_')[1].split('.zip')[0]));

    simulationIds.sort();

    let simulationId = 1;

    if (simulationIds.length) {
        simulationId = simulationIds[simulationIds.length - 1] + 1;
    }

    if (!constants) {
        constants = defaultConstants;
    }

    const jsonSimulation = {
        simulation_id: simulationId,
        constants: constants,
        cycle_ids: [],
    };

    let simulation = protobuf.serialize('Simulation', jsonSimulation);
    simulation = zlib.deflateSync(simulation);

    const simPath = path.join(simsPath, `simulation_${simulationId}`);

    fs.mkdirSync(simPath);
    fs.writeFileSync(path.join(simPath, 'simulation.zip'), simulation, { flag: 'wx'});

    return res.json(jsonSimulation);
}

const pipeline = [
    createSimulation
];

module.exports = pipeline;