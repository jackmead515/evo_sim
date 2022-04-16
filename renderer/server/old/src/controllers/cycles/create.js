const path = require('path');
const fs = require('fs');
const zlib = require('zlib');
const axios = require('axios');

const simsPath = path.join(__dirname, '../../../../../simulations');
const protobuf = require('../../protobuf');

async function createCycle(req, res) {
    try {
        const { simulationId } = req.params;

        const response = await axios.post(
            `http://127.0.0.1:8000/simulations/${simulationId}/cycles`,
            { responseType: 'arraybuffer' }
        );

        const cycle = protobuf.deserialize('Cycle', response.data);

        return res.json(cycle);
    } catch (err) {
        console.error(err);
        return res.status(500).end();
    }
}


const pipeline = [
    createCycle
];

module.exports = pipeline;