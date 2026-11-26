import Autocomplete from '@mui/material/Autocomplete';
import TextField from '@mui/material/TextField';
import { useLookups } from '../LookupsContext';

// Free-text name box that also suggests people already entered on the debt pages.
// Typing a new name is fine; picking a suggestion just fills it in.
//   value / onChange   the text (onChange receives a string)
//   textFieldProps     passed to the underlying TextField (label, margin, variant, ...)
export default function PersonInput({ value, onChange, textFieldProps = {}, ...autocompleteProps }) {
  const { people, refresh } = useLookups();

  return (
    <Autocomplete
      freeSolo
      options={people}
      inputValue={value ?? ''}
      onInputChange={(_, newValue) => onChange(newValue)}
      onOpen={() => refresh()}
      {...autocompleteProps}
      renderInput={(params) => <TextField {...params} {...textFieldProps} />}
    />
  );
}
