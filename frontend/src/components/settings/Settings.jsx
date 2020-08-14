import React from 'react';
import Button from '@material-ui/core/Button';
import Menu from '@material-ui/core/Menu';
import MenuItem from '@material-ui/core/MenuItem';
import { toast } from 'react-toastify';
import ExportDialog from './ExportDialog';
import css from './settings.module.css';

export default class Settings extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      anchor: undefined,
      exportDialogOpen: false,
    };
  }

    onClick = (event) => {
      this.setState({ anchor: event.currentTarget });
    };

    onClose = () => {
      this.setState({ anchor: undefined });
    };

    import = () => {
      this.onClose();
      document.getElementById('import').click();
    };

    upload = (event) => {
      const file = event.target.files[0];
      const reader = new FileReader();
      reader.readAsBinaryString(file);
      reader.onloadend = () => {
        const json = JSON.parse(reader.result);
        // eslint-disable-next-line react/destructuring-assignment
        this.props.import(json);
      };
    };

    export = () => {
      this.onClose();
      const { route } = this.props;
      if (!route) {
        toast.error('Please create a route first');
        return;
      }
      this.setState({ exportDialogOpen: true });
    };

    download = (filename) => {
      // eslint-disable-next-line react/destructuring-assignment
      const json = JSON.stringify(this.props.export());
      const contentType = 'application/json;charset=utf-8;';
      const a = document.createElement('a');

      a.download = `${filename}.json`;
      a.href = `data:${contentType},${encodeURIComponent(json)}`;
      a.target = '_blank';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    }

    render() {
      const { anchor, exportDialogOpen } = this.state;
      return (
        <div className={css.settings}>
          <Button
            className={css.settingsButton}
            variant="contained"
            onClick={this.onClick}
          >
            Settings
          </Button>

          <Menu
            id="simple-menu"
            anchorEl={anchor}
            open={Boolean(anchor)}
            onClose={this.onClose}
          >
            <MenuItem onClick={this.import}>Import</MenuItem>
            <MenuItem onClick={this.export}>Export</MenuItem>
          </Menu>

          <input
            id="import"
            className={css.import}
            type="file"
            accept=".json"
            onChange={this.upload}
          />
          <ExportDialog
            open={exportDialogOpen}
            close={() => this.setState({ exportDialogOpen: false })}
            download={this.download}
          />
        </div>
      );
    }
}
