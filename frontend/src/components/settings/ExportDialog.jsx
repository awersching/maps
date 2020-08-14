import React from 'react';
import Button from '@material-ui/core/Button';
import Dialog from '@material-ui/core/Dialog';
import DialogTitle from '@material-ui/core/DialogTitle';
import DialogContent from '@material-ui/core/DialogContent';
import DialogContentText from '@material-ui/core/DialogContentText';
import TextField from '@material-ui/core/TextField';
import DialogActions from '@material-ui/core/DialogActions';

export default class ExportDialog extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      filename: '',
    };
  }

    onClose = () => {
      const { close } = this.props;
      close();
    };

    onOk = () => {
      const { close, download } = this.props;
      const { filename } = this.state;
      close();
      download(filename);
    }

    render() {
      const { open } = this.props;
      const { filename } = this.state;
      return (
        <div>
          <Dialog
            open={open}
            onClose={this.onClose}
            aria-labelledby="form-dialog-title"
          >
            <DialogTitle id="form-dialog-title">Export</DialogTitle>

            <DialogContent>
              <DialogContentText>
                Please enter a filename for export
              </DialogContentText>
              <TextField
                value={filename}
                onChange={(event) => this.setState({ filename: event.target.value })}
                autoFocus
                margin="dense"
                id="filename"
                label="Filename"
                fullWidth
              />
            </DialogContent>

            <DialogActions>
              <Button onClick={this.onClose} color="primary">
                Cancel
              </Button>
              <Button onClick={this.onOk} color="primary">
                Ok
              </Button>
            </DialogActions>
          </Dialog>
        </div>
      );
    }
}
