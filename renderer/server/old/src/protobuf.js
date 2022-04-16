const path = require('path');
const fs = require('fs');
const protobuf = require('protobufjs');

let messages = undefined;

function serialize(schema, object) {
    const Schema = messages.lookup(`models.${schema}`);
    return Schema.encode(object).finish();
}

function deserialize(schema, buffer) {
    const Schema = messages.lookup(`models.${schema}`);
    return Schema.decode(buffer);
}

function initialize() {
    const modelsPath = path.join(__dirname, '../../../models.proto');
    const root = new protobuf.Root({ keepCase: true });
    messages = protobuf.loadSync(modelsPath, root);

    
    //messages = protobuf.Root.fromJSON(require('../bundle.json'), root);
    //const buffer = fs.readFileSync(path.join(__dirname, '../../../models.proto'));
    //messages = pbfCompile(schema.parse(buffer));
}

module.exports = {
    initialize,
    serialize,
    deserialize,
}