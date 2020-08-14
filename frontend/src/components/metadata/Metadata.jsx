import React from 'react';
import ElevationGraph from './ElevationGraph';
import Table from './Table';
import { roadSurfaceColor, roadTypeColor } from './colors';
import css from './metadata.module.css';

export default class Metadata extends React.Component {
    curvatureScore = () => {
      const { route, distance } = this.props;
      const { score } = route.curvature;
      return (score / (distance / 1000.0)).toFixed(1);
    };

    render() {
      const { route, distance, setTracker } = this.props;
      return (
        <div>
          <div>
            <b>Curvature Score:</b>
            {' '}
            {route && this.curvatureScore()}
          </div>
          <div className={css.metadataContainer}>
            <b>Intersections:</b>
            {' '}
            {route && route.intersections}
          </div>

          <div className={css.metadataContainer}>
            <ElevationGraph
              route={route}
              setTracker={setTracker}
            />
          </div>

          <div className={css.tableContainer}>
            <Table
              route={route}
              distance={distance}
              title="Road Type"
              color={roadTypeColor}
              type={(edge) => edge.meta.highway}
            />
          </div>
          <Table
            route={route}
            distance={distance}
            title="Road Surface"
            color={roadSurfaceColor}
            type={(edge) => edge.meta.surface}
          />
        </div>
      );
    }
}
