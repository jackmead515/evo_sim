import React from 'react';

export default class Toolbar extends React.PureComponent {

  constructor(props) {
    super(props);

    this.state = {
      sideBarDisplay: 'none'
    }

    this.onClickCreate = this.onClickCreate.bind(this);
  }

  render() {
    return (
        <div className="toolbar">
          <div className="toolbar__select">
            <button onClick={this.onClickCreate}>Create</button>
            <button>Select</button>
          </div>

          <div className="toolbar__steps">
            <button>Previous Step</button>
            <input type="range" min="1" max="100" value="50"></input>
            <button>Next Step</button>
          </div>

          <div className="sidebar" style={{display: this.state.sideBarDisplay}}>
            <div className="sidebar__top">
              <button onClick={this.onClickCreate}>Close</button>
            </div>
            {this.renderSideBar()}
          </div>
        </div>
    )
  }

  renderSideBar() {
    return (
      <div>
        <p>Create New Simulation</p>
      </div>
    );
  }

  onClickCreate() {
    if (this.state.sideBarDisplay === 'none') {
      this.setState({ sideBarDisplay: 'grid' });
    } else {
      this.setState({ sideBarDisplay: 'none' });
    }
  }
}