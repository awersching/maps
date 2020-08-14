/* eslint react/no-unused-state: 0 */
import React from 'react';
import './app.css';
import { toast } from 'react-toastify';
import Osm from './Osm';
import Bar from './bar/Bar';

toast.configure();

export default class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      stops: [],
      route: undefined,
      time: '0 h 0 min',
      distance: 0,
      tracker: undefined,
    };
    this.remapRightClick();
  }

    remapRightClick = () => {
      document.oncontextmenu = () => {
        this.clearMap();
        return false;
      };
    }

    addStop = (stop) => {
      const { stops } = this.state;
      stops.push(stop);
      this.setState({ stops });
    };

    removeStop = (index) => {
      const { stops } = this.state;
      stops.splice(index, 1);
      this.setState({ stops });
    };

    importState = (data) => {
      // reset first to reset elevation graph
      this.setState({ route: undefined });
      this.setState(data);
    };

    exportState = () => this.state;

    setRoute = (route) => {
      // reset first to reset elevation graph
      this.setState({ route: undefined });
      this.setState({
        route,
        time: this.hhmm(route.time),
        distance: route.distance,
        tracker: undefined,
      });
    };

    clearMap = () => {
      this.setState({
        stops: [],
        route: undefined,
        time: '0h 0 min',
        distance: 0,
        tracker: undefined,
      });
    };

    setTracker = (coords) => {
      this.setState({ tracker: coords });
    }

    hhmm = (secs) => {
      const hours = Math.floor(secs / 3600);
      const minutes = Math.floor((secs - (hours * 3600)) / 60);
      return `${hours} h ${minutes} min`;
    };

    render() {
      const {
        stops, route, time, distance, tracker,
      } = this.state;
      return (
        <div>
          <Osm
            stops={stops}
            route={route}
            tracker={tracker}
            addStop={this.addStop}
          />
          <Bar
            stops={stops}
            route={route}
            time={time}
            distance={distance}
            importState={this.importState}
            exportState={this.exportState}
            setRoute={this.setRoute}
            addStop={this.addStop}
            clearMap={this.clearMap}
            removeStop={this.removeStop}
            setTracker={this.setTracker}
          />
        </div>
      );
    }
}
