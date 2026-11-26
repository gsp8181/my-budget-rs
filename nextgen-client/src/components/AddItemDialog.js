import Alert from '@mui/material/Alert';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import TextField from '@mui/material/TextField';
import { useLookups } from '../LookupsContext';
import { cardLabel, currencyLabel, lookupOptions } from './lookupOptions';

// The "Add <item>" form. Inputs come from the page's `fields` in pageConfig.js;
// fields of type 'currency' or 'card' become dropdowns of the shared lookups.
export default function AddItemDialog({ open, title, fields, values, onChange, onSubmit, onClose }) {
  const { currencies, cards, status, refresh } = useLookups();

  const setValue = (name) => (e) => onChange({ ...values, [name]: e.target.value });

  const renderField = (field, idx) => {
    const common = {
      label: field.label,
      value: values[field.name] ?? '',
      onChange: setValue(field.name),
      fullWidth: true,
      margin: 'normal',
      autoFocus: idx === 0,
    };

    if (field.type === 'currency' || field.type === 'card') {
      const options = field.type === 'currency'
        ? lookupOptions(currencies, { what: 'currencies', status, label: currencyLabel })
        : lookupOptions(cards, { what: 'cards', status, label: cardLabel });
      return (
        <TextField key={field.name} {...common} select SelectProps={{ onOpen: () => refresh() }}>
          {options}
        </TextField>
      );
    }

    const isLast = idx === fields.length - 1;
    return (
      <TextField
        key={field.name}
        {...common}
        type={field.type || 'text'}
        inputProps={field.inputProps}
        placeholder={field.placeholder}
        onKeyDown={isLast ? (e) => { if (e.key === 'Enter') onSubmit(); } : undefined}
      />
    );
  };

  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle>{title}</DialogTitle>
      <DialogContent>
        {status === 'error' && (
          <Alert severity="warning" sx={{ mt: 1 }}>
            Couldn't reach the server to load currencies and cards. Retrying automatically…
          </Alert>
        )}
        {fields.map(renderField)}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button variant="contained" onClick={onSubmit}>Add</Button>
      </DialogActions>
    </Dialog>
  );
}
