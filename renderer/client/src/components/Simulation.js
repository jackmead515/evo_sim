import React from 'react';

import * as simulation from '../sim/engine';

export default class AppContainer extends React.PureComponent {

  componentDidMount() {
		const element = document.getElementById('simulation');
		enableDragElement(element);
    this.createEngine(element);
    centerElement(element);
	}

  createEngine(element) {
    const engine = simulation.createEngine(element);
   
    function animate() {
      requestAnimationFrame(animate);
      engine.renderer.render(engine.scene, engine.camera);
      engine.update();
    }
    animate();
  }

  render() {
    return (
      <div className="simulation">
        <div id="simulation" className="simulation__view">
        </div>
      </div>
    )
  }
}

function centerElement(element) {
 
}

export function enableDragElement(element) {
  const pos = [0, 0, 0, 0];

  element.onmousedown = dragMouseDown;

  function dragMouseDown(e) {
    e = e || window.event;
    e.preventDefault();
    if (e.which === 2) {
      pos[2] = e.clientX;
      pos[3] = e.clientY;
      document.onmouseup = closeDragElement;
      document.onmousemove = elementDrag;
    }
  }

  function elementDrag(e) {
    e = e || window.event;
    e.preventDefault();
    if (e.which === 2) {
      pos[0] = pos[2] - e.clientX;
      pos[1] = pos[3] - e.clientY;
      pos[2] = e.clientX;
      pos[3] = e.clientY;
      element.style.top = (element.offsetTop - pos[1]) + 'px';
      element.style.left = (element.offsetLeft - pos[0]) + 'px';
    }
  }

  function closeDragElement() {
    document.onmouseup = null;
    document.onmousemove = null;
  }
}