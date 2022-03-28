import React from 'react';
import Toolbar from './components/Toolbar';
import Simulation from './components/Simulation';

export default class AppContainer extends React.PureComponent {
  render() {
    return (
      <div className="container">
        <Toolbar />
        <div className="container__simwrapper">
          <Simulation />
        </div>
      </div>
    )
  }
}