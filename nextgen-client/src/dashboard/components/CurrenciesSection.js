import { useEffect, useState, useCallback } from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Typography from '@mui/material/Typography';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import TextField from '@mui/material/TextField';
import IconButton from '@mui/material/IconButton';
import AddIcon from '@mui/icons-material/Add';
import DeleteIcon from '@mui/icons-material/DeleteOutlined';
import Alert from '@mui/material/Alert';
import CircularProgress from '@mui/material/CircularProgress';
import API_BASE from '../../config';

export default function CurrenciesSection() {
  const [currencies, setCurrencies] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [newCurrency, setNewCurrency] = useState({ rate: '', symbol: '', name: '' });
  const [adding, setAdding] = useState(false);

  const fetchCurrencies = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_BASE}/api/currency`);
      if (!res.ok) throw new Error(`Failed to fetch currencies (${res.status})`);
      setCurrencies(await res.json());
    } catch (e) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { fetchCurrencies(); }, [fetchCurrencies]);

  const handleAdd = async () => {
    if (!newCurrency.rate || !newCurrency.symbol || !newCurrency.name) {
      setError('Rate, symbol, and name are all required.');
      return;
    }
    setAdding(true);
    try {
      const res = await fetch(`${API_BASE}/api/currency`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newCurrency),
      });
      if (!res.ok) throw new Error(`Failed to add currency (${res.status})`);
      setNewCurrency({ rate: '', symbol: '', name: '' });
      fetchCurrencies();
    } catch (e) {
      setError(e.message);
    } finally {
      setAdding(false);
    }
  };

  const handleDelete = async (id) => {
    try {
      const res = await fetch(`${API_BASE}/api/currency/${id}`, { method: 'DELETE' });
      if (res.status === 409) {
        const msg = await res.text();
        throw new Error(msg || 'Cannot delete: currency is linked to items.');
      }
      if (!res.ok) throw new Error(`Failed to delete currency (${res.status})`);
      fetchCurrencies();
    } catch (e) {
      setError(e.message);
    }
  };

  const handleInlineChange = (id, field, value) => {
    setCurrencies(prev => prev.map(c => c.id === id ? { ...c, [field]: value } : c));
  };

  const handleInlineSave = async (currency) => {
    try {
      const res = await fetch(`${API_BASE}/api/currency/${currency.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ rate: currency.rate, symbol: currency.symbol, name: currency.name }),
      });
      if (!res.ok) throw new Error(`Failed to update currency (${res.status})`);
    } catch (e) {
      setError(e.message);
      fetchCurrencies();
    }
  };

  if (loading) return <CircularProgress size={24} />;

  return (
    <Box>
      <Typography variant="subtitle1" sx={{ mb: 1, fontWeight: 600 }}>Currencies</Typography>
      {error && (
        <Alert severity="error" onClose={() => setError(null)} sx={{ mb: 1 }}>
          {error}
        </Alert>
      )}
      <Table size="small" sx={{ mb: 2 }}>
        <TableHead>
          <TableRow>
            <TableCell>Symbol</TableCell>
            <TableCell>Name</TableCell>
            <TableCell>Rate to £</TableCell>
            <TableCell />
          </TableRow>
        </TableHead>
        <TableBody>
          {currencies.map((c) => (
            <TableRow key={c.id}>
              <TableCell>
                <TextField
                  size="small"
                  value={c.symbol}
                  onChange={e => handleInlineChange(c.id, 'symbol', e.target.value)}
                  onBlur={() => handleInlineSave(c)}
                  inputProps={{ style: { width: 60 } }}
                />
              </TableCell>
              <TableCell>
                <TextField
                  size="small"
                  value={c.name}
                  onChange={e => handleInlineChange(c.id, 'name', e.target.value)}
                  onBlur={() => handleInlineSave(c)}
                  inputProps={{ style: { width: 180 } }}
                />
              </TableCell>
              <TableCell>
                <TextField
                  size="small"
                  value={c.rate}
                  onChange={e => handleInlineChange(c.id, 'rate', e.target.value)}
                  onBlur={() => handleInlineSave(c)}
                  inputProps={{ style: { width: 80 } }}
                />
              </TableCell>
              <TableCell padding="none">
                <IconButton
                  size="small"
                  color="error"
                  onClick={() => handleDelete(c.id)}
                  title="Delete currency"
                >
                  <DeleteIcon fontSize="small" />
                </IconButton>
              </TableCell>
            </TableRow>
          ))}
          {/* Add new row */}
          <TableRow>
            <TableCell>
              <TextField
                size="small"
                placeholder="€"
                value={newCurrency.symbol}
                onChange={e => setNewCurrency(p => ({ ...p, symbol: e.target.value }))}
                inputProps={{ style: { width: 60 } }}
              />
            </TableCell>
            <TableCell>
              <TextField
                size="small"
                placeholder="Euro"
                value={newCurrency.name}
                onChange={e => setNewCurrency(p => ({ ...p, name: e.target.value }))}
                inputProps={{ style: { width: 180 } }}
              />
            </TableCell>
            <TableCell>
              <TextField
                size="small"
                placeholder="0.86"
                value={newCurrency.rate}
                onChange={e => setNewCurrency(p => ({ ...p, rate: e.target.value }))}
                inputProps={{ style: { width: 80 } }}
              />
            </TableCell>
            <TableCell padding="none">
              <IconButton
                size="small"
                color="primary"
                onClick={handleAdd}
                disabled={adding}
                title="Add currency"
              >
                <AddIcon fontSize="small" />
              </IconButton>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
      <Typography variant="caption" color="text.secondary">
        Rate is how many units of this currency equal £1 (e.g. Euro ≈ 1.28 means £1 = €1.28 → rate = 1.28). British Pound (rate 1) is always present and cannot be removed.
      </Typography>
    </Box>
  );
}
