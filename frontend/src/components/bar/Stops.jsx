import React from 'react';
import IconButton from '@material-ui/core/IconButton';
import { Delete } from '@material-ui/icons';
import { TextField } from '@material-ui/core';
import css from './bar.module.css';

export default class Stops extends React.Component {
  render() {
    const { stops, removeStop } = this.props;
    return (
      <div>
        {stops.map((stop, index) => (
          <div key={stop.coordinates.toString()} className={css.stop}>
            <TextField
              className={css.stopTextField}
              InputProps={{ className: css.stopTextField }}
              variant="outlined"
              value={stop.name}
            />
            <IconButton onClick={() => removeStop(index)}>
              <Delete />
            </IconButton>
          </div>
        ))}
      </div>
    );
  }
}
