import React from 'react';
import Button from '@material-ui/core/Button';
import css from './bar.module.css';
import Settings from '../settings/Settings';

export default class ButtonGroup extends React.Component {
  render() {
    const {
      route, clearMap, go, importState, exportState,
    } = this.props;
    return (
      <div className={css.buttonGroup}>
        <Settings
          route={route}
          importState={importState}
          exportState={exportState}
        />

        <div className={css.buttonContainer}>
          <Button
            className={css.reset}
            variant="contained"
            onClick={clearMap}
          >
            Reset
          </Button>
        </div>

        <div className={css.buttonContainer}>
          <Button
            className={css.go}
            variant="contained"
            onClick={go}
          >
            Go
          </Button>
        </div>
      </div>
    );
  }
}
