import React from 'react';
import MuiTable from '@material-ui/core/Table';
import TableRow from '@material-ui/core/TableRow';
import TableCell from '@material-ui/core/TableCell';
import TableBody from '@material-ui/core/TableBody';
import FiberManualRecordIcon from '@material-ui/icons/FiberManualRecord';
import css from './metadata.module.css';

export default class Table extends React.Component {
    percentages = () => {
      const data = this.data();
      const percentages = [];

      data.forEach((d) => {
        const { distance } = this.props;
        percentages.push({ color: d.color, percentage: (d.distance / distance) * 100 });
      });
      return percentages;
    }

    data = () => {
      const { route, type, color } = this.props;
      if (!route) {
        return [];
      }
      const { edges } = route;

      const data = {};
      edges.forEach((edge) => {
        let edgeType = type(edge);
        if (!edgeType) {
          edgeType = 'Unknown';
        }
        // e.g. TrunkLink -> Trunk
        edgeType = edgeType.replace('Link', '');

        if (edgeType in data) {
          data[edgeType].distance += edge.distance;
        } else {
          data[edgeType] = {
            color: color(edgeType),
            type: this.splitAtCapitalLetter(edgeType),
            distance: edge.distance,

          };
        }
      });
      return this.sort(Object.values(data));
    };

    sort = (data) => {
      data.sort((a, b) => {
        if (parseFloat(a.distance) < parseFloat(b.distance)) {
          return 1;
        }
        if (parseFloat(a.distance) > parseFloat(b.distance)) {
          return -1;
        }
        return 0;
      });
      return data;
    };

    splitAtCapitalLetter = (string) => string.replace(/([A-Z])/g, ' $1')

    render() {
      const { title } = this.props;
      return (
        <div>
          <div className={css.tableContent}>
            <b>{title}</b>
          </div>

          <div className={css.tableContent}>
            <div className={css.coloredBar}>
              {this.percentages().map(({ color, percentage }) => (
                <div style={{
                  height: '100%',
                  width: `${percentage}%`,
                  background: color,
                  display: 'inline-block',
                }}
                />
              ))}
            </div>
          </div>

          <MuiTable size="small" aria-label="a dense table">
            <TableBody>
              {this.data().map(({ color, type, distance }) => (
                <TableRow>
                  <TableCell className={css.tableCell}>
                    <FiberManualRecordIcon style={{ color }} />
                  </TableCell>
                  <TableCell>{type}</TableCell>
                  <TableCell>
                    {(distance / 1000).toFixed(2)}
                    {' '}
                    km
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </MuiTable>
        </div>
      );
    }
}
