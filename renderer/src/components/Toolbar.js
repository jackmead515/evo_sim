import React from 'react';

export default class Toolbar extends React.PureComponent {
  render() {
    return (
        <div className="toolbar">
          <div className="toolbar__select">
            <button>Select Simulation</button>
            <button>Select Cycle</button>
          </div>
          <div className="toolbar__steps">
            <button>Previous Step</button>
            <input type="range" min="1" max="100" value="50"></input>
            <button>Next Step</button>
          </div>
        </div>
    )
  }
}