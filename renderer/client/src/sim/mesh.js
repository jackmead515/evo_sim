import * as three from 'three';

export function createBoxMesh() {
    const geometry = new three.BoxGeometry( 0.2, 0.2, 0.1 );
    const material = new three.MeshBasicMaterial({ color: 0x00ff00 });
    return new three.Mesh( geometry, material );
}

export function createLineMesh() {
    const material = new three.LineBasicMaterial({ color: 0x00ff00 });
    const geometry = new three.BufferGeometry().setFromPoints([
        new three.Vector3( -0.1, 0, 0 ),
        new three.Vector3( 0, 0.1, 0 ),
        new three.Vector3( 0.1, 0, 0 )
    ]);
    return new three.Line( geometry, material );
}