const path = require('path');
const fs = require('fs');
const zlib = require('zlib');

const protobuf = require('../../protobuf');

const simsPath = path.join(__dirname, '../../../../../simulations');

function listSimulations(req, res) {

    const Simulation = protobuf.get('models.Simulation');

    const simulations = fs.readdirSync(simsPath)
        .filter((file) => file.startsWith('simulation_'))
        .map((file) => path.join(simsPath, file, 'simulation.zip'))
        .map((file) => {
            const buffer = zlib.inflateSync(fs.readFileSync(file));
            const simulation = Simulation.decode(buffer);
            return Simulation.toObject(simulation);
        });
    
    return res.json(simulations);
}

const pipeline = [
    listSimulations
];

module.exports = pipeline;