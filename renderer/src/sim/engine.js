import * as three from 'three';
import * as dat from 'dat-gui';
import * as mesh from './mesh';

export function createEngine(element) {
    const engine = {};
    const gui = new dat.GUI();

    const guiControl = new class {
      yRotation = 0;
    }();

    element.parentElement.appendChild(gui.domElement);
    gui.domElement.style.position = 'absolute';
    gui.domElement.style.right = '0';
    gui.add(guiControl, 'yRotation', 0.0, 1.0);

    const world_width = 1000;
    const world_height = 1000;

    element.style.width = world_width;
    element.style.height = world_height;
    engine.camera = new three.PerspectiveCamera(10,world_width/world_height,0.01,10);
    engine.camera.position.z = 3;
    engine.scene = new three.Scene();
    engine.renderer = new three.WebGLRenderer({ antialias: true });
    engine.renderer.setSize(world_width, world_height);
    element.appendChild(engine.renderer.domElement);

    //const boxMesh = createBoxMesh();
    //const lineMesh = createLineMesh();

    const meshes = [];
    for (let i = 0; i < 10; i++) {
      meshes.push(mesh.createLineMesh());
    }
    for (const mesh of meshes) {
      engine.scene.add(mesh);
    }

    engine.update = function() {
        for (const mesh of meshes) {
            //mesh.rotation.x += Math.random() * 0.01;
            mesh.rotation.y += guiControl.yRotation;
            //mesh.rotation.z += Math.random() * 0.01;
        }
    }

    return engine;
}