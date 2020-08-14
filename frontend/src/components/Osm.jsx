import React from 'react';
import axios from 'axios';
import { toast } from 'react-toastify';
import {
  CircleMarker, LayersControl, Map, Marker, Polyline, Popup, TileLayer, ZoomControl,
} from 'react-leaflet';

const OSM = {
  url: 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png',
  attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
};
const TOPOLOGY = {
  url: 'https://tiles.wmflabs.org/hillshading/{z}/{x}/{y}.png',
  attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
};
const SATELLITE = {
  url: 'http://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}',
  attribution: 'Service &copy <a href=https://heigit.org> HeiGIT</a> @ '
        + '<a href="https://www.uni-heidelberg.de/">Heidelberg University</a>',
};

export default class Osm extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      center: undefined,
    };
    this.locate();
  }

    locate = () => {
      axios.get('http://ip-api.com/json/?fields=lat,lon')
        .then((position) => {
          this.setState({ center: Object.values(position.data) });
          // don't recenter on location after clear
          this.setState({ center: undefined });
        }).catch(() => toast.error('Unable to retrieve current location'));
    };

    center = () => {
      const { center } = this.state;
      const { stops } = this.props;

      if (center) {
        return center;
      }
      if (stops.length > 0) {
        const lastStop = stops[stops.length - 1];
        if (this.isSearchResult(lastStop)) {
          return lastStop.coordinates;
        }
      }
      return undefined;
    };

    isSearchResult = (stop) => stop.name !== stop.coordinates.toString();

    bounds = () => {
      const { route } = this.props;
      if (route) {
        return route.nodes.map((node) => node.coordinates);
      }
      return undefined;
    };

    setMarker = (event) => {
      const { addStop } = this.props;
      const coordinates = Object.values(event.latlng);
      const stop = {
        name: coordinates.toString(),
        coordinates,
      };
      addStop(stop);
    };

    polylines = () => {
      const { route } = this.props;
      if (!route) {
        return [];
      }
      const { nodes } = route;
      // too big to render individual radii efficiently
      if (nodes.length > 1000) {
        const singleLine = { positions: nodes.map((n) => n.coordinates), color: 'blue', weight: 3 };
        return [singleLine];
      }

      const { radii } = route.curvature;
      const lines = [];
      for (let i = 0; i < nodes.length - 1; i += 1) {
        const n1 = nodes[i];
        const n2 = nodes[i + 1];
        const radius = radii[i];

        const positions = [n1.coordinates, n2.coordinates];
        lines.push({ positions, color: this.color(radius), weight: 7 });
      }
      return lines;
    };

    color = (radius) => {
      if (!radius) {
        return '#92d050';
      }

      if (radius < 160) {
        return '#ff0000';
      }
      if (radius < 170) {
        return '#ffc000';
      }
      if (radius < 175) {
        return '#ffff00';
      }
      return '#92d050';
    }

    render() {
      const { stops, tracker } = this.props;
      return (
        <Map
          center={this.center()}
          bounds={this.bounds()}
          onClick={this.setMarker}
          zoom={16}
          zoomControl={false}
        >
          <ZoomControl position="topright" />

          <LayersControl>
            <LayersControl.BaseLayer name="OpenStreetMap" checked>
              <TileLayer url={OSM.url} attribution={OSM.attribution} />
            </LayersControl.BaseLayer>
            <LayersControl.Overlay name="Topology" checked>
              <TileLayer url={TOPOLOGY.url} attribution={TOPOLOGY.attribution} />
            </LayersControl.Overlay>
            <LayersControl.BaseLayer name="Satellite">
              <TileLayer url={SATELLITE.url} attribution={SATELLITE.attribution} />
            </LayersControl.BaseLayer>
          </LayersControl>

          {tracker && <CircleMarker center={tracker} />}
          {stops.map((stop) => (
            <Marker key={stop.coordinates.toString()} position={stop.coordinates}>
              <Popup>
                <b>{stop.name}</b>
                <br />
                {stop.coordinates}
              </Popup>
            </Marker>
          ))}
          {this.polylines().map(({ positions, color, weight }) => (
            <Polyline
              positions={positions}
              color={color}
              weight={weight}
            />
          ))}
        </Map>
      );
    }
}
