const express = require('express');
const path = require('path');

const protobuf = require('./protobuf');

async function run() {

    protobuf.initialize();

    const fs = require('fs');

    const buffer = fs.readFileSync('../../simulations/simulation_1/cycles/cycle_1.zip', 'utf-8');
    const data = protobuf.deserialize('Cycle', new Uint8Array(buffer));
    console.log(data);

    // const app = express();

    // const webRoute = express.static(path.join(__dirname, '../../client/build'));

    // app.use('/', webRoute);

    // app.get('/api/simulations', require('./controllers/simulations/list'));
    // app.post('/api/simulations', require('./controllers/simulations/create'));

    // app.post('/api/simulations/:simulationId/cycles', require('./controllers/cycles/create'));

    // app.listen(80);
}

run();