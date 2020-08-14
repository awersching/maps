import React from 'react';
import { Bar } from 'react-chartjs-2';
import legend from './legend.png';
import css from './metadata.module.css';

const ITEM_LIMIT = 300;

export default class ElevationGraph extends React.Component {
  constructor(props) {
    super(props);
    this.chart = undefined;
  }

    data = () => {
      const { route } = this.props;
      if (!route) {
        return [];
      }

      const { nodes } = route;
      const { labels, items, backgroundColors } = nodes.length <= ITEM_LIMIT
        ? this.rawData() : this.smoothedData();
      return { labels, datasets: [{ label: 'Elevation (m)', data: items, backgroundColor: backgroundColors }] };
    }

    rawData = () => {
      const { route } = this.props;
      const { nodes, edges } = route;
      const labels = [];
      const items = [];
      const backgroundColors = [];

      for (let i = 0; i < nodes.length; i += 1) {
        const node = nodes[i];
        const edge = i <= edges.length - 1 ? edges[i] : edges[edges.length - 1];

        labels.push(`${node.coordinates.lat}, ${node.coordinates.lon}`);
        items.push(node.meta.elevation.toFixed(1));
        backgroundColors.push(this.color(edge.meta.grade));
      }
      return { labels, items, backgroundColors };
    };

    smoothedData = () => {
      const { route } = this.props;
      const { nodes, edges } = route;
      const labels = [];
      const items = [];
      const backgroundColors = [];

      const smoothing = Math.round(nodes.length / ITEM_LIMIT);
      for (let i = 0; i < nodes.length - smoothing; i += smoothing) {
        const elevations = nodes.slice(i, i + smoothing)
          .map((n) => n.meta.elevation);
        const avgElevation = (elevations.reduce((e1, e2) => e1 + e2, 0) / elevations.length);
        const grades = edges.slice(i, i + smoothing)
          .map((e) => e.meta.grade);
        const avgGrade = Math.round(grades.reduce((e1, e2) => e1 + e2, 0) / grades.length);

        labels.push(`${nodes[i].coordinates.lat}, ${nodes[i].coordinates.lon}`);
        items.push(avgElevation.toFixed(1));
        backgroundColors.push(this.color(avgGrade));
      }
      return { labels, items, backgroundColors };
    };

    color = (grade) => {
      if (grade < 1) {
        return '#d9d9d9';
      }
      if (grade < 3) {
        return '#92d050';
      }
      if (grade < 6) {
        return '#ffff00';
      }
      if (grade < 9) {
        return '#ffc000';
      }
      return '#ff0000';
    };

    onHover = (event) => {
      const item = this.chart.chartInstance.getElementAtEvent(event);
      if (item.length === 0) {
        return;
      }

      // eslint-disable-next-line no-underscore-dangle
      const graphIndex = item[0]._index;
      const { route, setTracker } = this.props;
      const { nodes } = route;
      let coords;
      if (nodes.length <= ITEM_LIMIT) {
        coords = nodes[graphIndex].coordinates;
      } else {
        const smoothing = Math.round(nodes.length / ITEM_LIMIT);
        if (graphIndex * smoothing < nodes.length) {
          coords = nodes[graphIndex * smoothing].coordinates;
        } else {
          coords = nodes[nodes.length - 1].coordinates;
        }
      }
      setTracker(coords);
    };

    options = () => ({
      onHover: this.onHover,
      legend: {
        display: false,
      },
      scales: {
        xAxes: [{ display: false }],
      },
      animation: {
        duration: 500,
      },
      hover: {
        animationDuration: 0,
      },
      responsiveAnimationDuration: 0,
      ticks: {
        sampleSize: {
          min: 0,
          max: 0,
        },
        minRotation: 0,
        maxRotation: 0,
      },
    })

    render() {
      return (
        <div>
          <Bar
            ref={(reference) => {
              this.chart = reference;
            }}
            data={this.data}
            options={this.options()}
          />
          <img className={css.legend} src={legend} alt="" />
        </div>
      );
    }
}
