import React from 'react';
import axios from 'axios';
import Autosuggest from 'react-autosuggest';
import Divider from '@material-ui/core/Divider';
import css from './searchbox.module.css';

const NOMINATIM_API = 'https://nominatim.openstreetmap.org';

export default class SearchBox extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      value: '',
      suggestions: [],
    };
  }

    renderSuggestion = (suggestion) => (
      <div>
        <Divider />
        <div className={css.suggestionItem}>
          {suggestion.properties.display_name}
        </div>
      </div>
    );

    onSuggestionSelected = (event, { suggestion }) => {
      this.onSuggestionsClearRequested();
      const name = suggestion.properties.display_name;
      const coordinates = suggestion.geometry.coordinates.reverse();
      const { addStop } = this.props;
      addStop({ name, coordinates });
      this.setState({ value: '' });
    };

    onSuggestionsFetchRequested = ({ value }) => {
      const url = `${NOMINATIM_API}/search/?q=${value}&format=geojson&countrycodes=de`;
      axios.post(url)
        .then((response) => response.data)
        .then((data) => this.setState({ suggestions: data.features.slice(0, 5) }));
    };

    onSuggestionsClearRequested = () => {
      this.setState({ suggestions: [] });
    };

    shouldRenderSuggestions = (value) => value.trim().length > 2;

    getSuggestionValue = (suggestion) => suggestion.properties.display_name;

    onChange = (event, { newValue }) => {
      this.setState({ value: newValue });
    };

    render() {
      const { suggestions, value } = this.state;
      const inputProps = {
        placeholder: 'Search',
        value,
        onChange: this.onChange,
      };

      return (
        <Autosuggest
          theme={css}
          suggestions={suggestions}
          onSuggestionsFetchRequested={this.onSuggestionsFetchRequested}
          onSuggestionsClearRequested={this.onSuggestionsClearRequested}
          getSuggestionValue={this.getSuggestionValue}
          renderSuggestion={this.renderSuggestion}
          shouldRenderSuggestions={this.shouldRenderSuggestions}
          onSuggestionSelected={this.onSuggestionSelected}
          inputProps={inputProps}
        />
      );
    }
}
