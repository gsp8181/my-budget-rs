import MenuItem from '@mui/material/MenuItem';

// Options for a currency/card dropdown, or a single disabled placeholder that
// says why the list is empty. Returned as an array (not a component) because
// MUI's Select needs its MenuItems as direct children.
export function lookupOptions(items, { what, status, label }) {
  if (items.length > 0) {
    return items.map((item) => (
      <MenuItem key={item.id} value={item.id}>{label(item)}</MenuItem>
    ));
  }
  let text = `No ${what} yet`;
  if (status === 'error') text = `Couldn't load ${what} - retrying…`;
  else if (status === 'loading') text = `Loading ${what}…`;
  return [<MenuItem key="empty" disabled value="">{text}</MenuItem>];
}

export const currencyLabel = (c) => `${c.symbol} — ${c.name}`;
export const currencySymbol = (c) => c.symbol;
export const cardLabel = (c) => c.name;
