import React from 'react';
import ToggleButtonGroup from '@material-ui/lab/ToggleButtonGroup';
import ToggleButton from '@material-ui/lab/ToggleButton';
import { DirectionsBike, DirectionsCar, DirectionsWalk } from '@material-ui/icons';
import RadioGroup from '@material-ui/core/RadioGroup';
import FormControlLabel from '@material-ui/core/FormControlLabel';
import Radio from '@material-ui/core/Radio';
import Checkbox from '@material-ui/core/Checkbox';
import axios from 'axios';
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import Typography from '@material-ui/core/Typography';
import Accordion from '@material-ui/core/Accordion';
import AccordionDetails from '@material-ui/core/AccordionDetails';
import AccordionSummary from '@material-ui/core/AccordionSummary';
import Metadata from '../metadata/Metadata';
import Stops from './Stops';
import SearchBox from './SearchBox';
import css from './bar.module.css';
import BASE_URL from '../../config';
import ButtonGroup from './ButtonGroup';

const CAR = 'car';
const BIKE = 'bike';
const WALK = 'walk';
const TIME = 'time';
const DISTANCE = 'distance';

export default class Bar extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      transport: CAR,
      routing: TIME,
      avoidUnpaved: false,
      disableTime: false,
    };
  }

    setTransport = (_, newTransport) => {
      const { routing } = this.state;
      let rtng;
      let disableTime;

      if (newTransport === CAR) {
        disableTime = false;
        rtng = routing;
      } else if (newTransport === BIKE || newTransport === WALK) {
        disableTime = true;
        rtng = DISTANCE;
      }
      this.setState({
        transport: newTransport,
        routing: rtng,
        disableTime,
      });
    };

    setRouting = (event) => {
      this.setState({ routing: event.target.value });
    };

    go = () => {
      const { stops, setRoute } = this.props;
      if (stops.length < 2) {
        toast.error('Please select at least 2 stops');
        return;
      }

      const { transport, routing, avoidUnpaved } = this.state;
      const coords = stops.map((stop) => ({ lat: stop.coordinates[0], lon: stop.coordinates[1] }));
      const data = {
        stops: coords,
        transport,
        routing,
        avoid_unpaved: avoidUnpaved,
      };

      axios.post(`${BASE_URL}/shortest-path`, data).then((response) => {
        setRoute(response.data);
      }).catch((err) => {
        if (err.response) {
          toast.error(err.response.data);
        } else {
          toast.error('Connection error');
        }
      });
    };

    round = (value) => Math.round(value * 10) / 10;

    render() {
      const {
        transport, routing, avoidUnpaved, disableTime,
      } = this.state;
      const {
        stops, route, time, distance, removeStop, addStop,
        clearMap, setTracker, importState, exportState,
      } = this.props;
      return (
        <div className={css.bar}>
          <div className={css.navigation}>
            <div className={css.navigationSettings}>
              <ToggleButtonGroup
                value={transport}
                exclusive
                onChange={this.setTransport}
              >
                <ToggleButton value={CAR}>
                  <DirectionsCar />
                </ToggleButton>
                <ToggleButton value={BIKE}>
                  <DirectionsBike />
                </ToggleButton>
                <ToggleButton value={WALK}>
                  <DirectionsWalk />
                </ToggleButton>
              </ToggleButtonGroup>

              <RadioGroup
                value={routing}
                onChange={this.setRouting}
              >
                <div className={css.radioButtons}>
                  <FormControlLabel
                    value={TIME}
                    disabled={disableTime}
                    control={<Radio color="primary" />}
                    label="Time"
                    labelPlacement="start"
                  />
                  <FormControlLabel
                    value={DISTANCE}
                    control={<Radio color="primary" />}
                    label="Distance"
                    labelPlacement="start"
                  />
                </div>
              </RadioGroup>
            </div>
            <div>
              <Checkbox
                checked={avoidUnpaved}
                value={avoidUnpaved}
                onChange={() => this.setState({ avoidUnpaved: !avoidUnpaved })}
                color="primary"
              />
              Avoid unpaved roads
            </div>

            <Stops
              stops={stops}
              removeStop={removeStop}
            />
            <SearchBox addStop={addStop} />

            <ButtonGroup
              route={route}
              clearMap={clearMap}
              importState={importState}
              exportState={exportState}
              go={this.go}
            />
            <div className={css.timeDistanceText}>
              {time}
              {' '}
              |
              {' '}
              {this.round(distance / 1000)}
              {' '}
              km
            </div>
          </div>

          <Accordion
            defaultExpanded={false}
            expanded={distance !== 0}
          >
            <AccordionSummary
              expandIcon={<ExpandMoreIcon />}
              aria-controls="panel1a-content"
              id="panel1a-header"
            >
              <Typography>Metadata</Typography>
            </AccordionSummary>
            <AccordionDetails>
              <Metadata
                route={route}
                distance={distance}
                setTracker={setTracker}
              />
            </AccordionDetails>
          </Accordion>
        </div>
      );
    }
}
