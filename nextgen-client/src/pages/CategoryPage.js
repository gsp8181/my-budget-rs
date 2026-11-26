import { useCallback, useEffect, useState } from 'react';
import Alert from '@mui/material/Alert';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CircularProgress from '@mui/material/CircularProgress';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import Select from '@mui/material/Select';
import AddIcon from '@mui/icons-material/Add';
import api from '../api';
import { useLookups } from '../LookupsContext';
import AddItemDialog from '../components/AddItemDialog';
import ConfirmDialog from '../components/ConfirmDialog';
import ItemsTable from '../components/ItemsTable';
import { currencyLabel } from '../components/lookupOptions';

// A list page for one category of budget item (bank accounts, regular payments, ...).
// Everything page-specific comes from that page's entry in pageConfig.js.
export default function CategoryPage({ page }) {
  const { apiPath, itemName, columns, fields } = page;
  const { currencies, refresh: refreshLookups } = useLookups();

  // Card balances feed the card dropdowns and debts feed the person suggestions,
  // so changing either means the shared lookups need reloading.
  const afterChange = useCallback(() => {
    if (['cardbalance', 'debt', 'debtto'].includes(apiPath)) refreshLookups();
  }, [apiPath, refreshLookups]);

  const [rows, setRows] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [currencyFilter, setCurrencyFilter] = useState('all');
  const [deleteId, setDeleteId] = useState(null);

  // ---- Loading ----
  const loadRows = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setRows(await api.get(`/${apiPath}`));
    } catch (e) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }, [apiPath]);

  useEffect(() => { loadRows(); }, [loadRows]);

  // ---- Editing (called by the table when a row edit is saved) ----
  const updateRow = useCallback(async (newRow, oldRow) => {
    const changed = Object.fromEntries(
      Object.entries(newRow).filter(([key, value]) => key !== 'id' && value !== oldRow[key]),
    );
    if (Object.keys(changed).length === 0) return oldRow;
    try {
      await api.put(`/${apiPath}/${newRow.id}`, changed);
      // Keep our copy in step with the table, or the edit is lost on the next re-render.
      setRows((prev) => prev.map((r) => (r.id === newRow.id ? newRow : r)));
      afterChange();
      return newRow;
    } catch (e) {
      setError(e.message);
      return oldRow;
    }
  }, [apiPath, afterChange]);

  // ---- Deleting ----
  const deleteRow = async (id) => {
    try {
      await api.delete(`/${apiPath}/${id}`);
      setRows((prev) => prev.filter((r) => r.id !== id));
      afterChange();
    } catch (e) {
      setError(e.message);
    } finally {
      setDeleteId(null);
    }
  };

  // ---- Adding ----
  // Default to the oldest currency (lowest id), which is always £.
  const defaultCurrencyId = currencies.length > 0 ? currencies[0].id : '';
  const emptyForm = () => ({
    ...Object.fromEntries(fields.map((f) => [f.name, ''])),
    currency_id: defaultCurrencyId,
  });
  const [addOpen, setAddOpen] = useState(false);
  const [formValues, setFormValues] = useState(emptyForm);

  const openAddDialog = () => {
    setFormValues(emptyForm());
    setAddOpen(true);
    refreshLookups();
  };

  // If the currencies only arrive after the dialog opened, fill in the default then.
  useEffect(() => {
    if (addOpen && defaultCurrencyId !== '') {
      setFormValues((prev) => (prev.currency_id === '' || prev.currency_id == null
        ? { ...prev, currency_id: defaultCurrencyId }
        : prev));
    }
  }, [addOpen, defaultCurrencyId]);

  const addItem = async () => {
    try {
      await api.post(`/${apiPath}`, formValues);
      setAddOpen(false);
      loadRows();
      afterChange();
    } catch (e) {
      setError(e.message);
    }
  };

  // ---- Render ----
  const visibleRows = currencyFilter === 'all'
    ? rows
    : rows.filter((r) => String(r.currency_id) === String(currencyFilter));

  return (
    <Box sx={{ width: '100%' }}>
      {error && (
        <Alert severity="error" onClose={() => setError(null)} sx={{ mb: 2 }}>
          {error}
        </Alert>
      )}
      <Box sx={{ mb: 2, display: 'flex', justifyContent: 'flex-end', alignItems: 'center', gap: 2 }}>
        {currencies.length > 1 && (
          <FormControl size="small" sx={{ minWidth: 140 }}>
            <InputLabel>Currency</InputLabel>
            <Select
              value={currencyFilter}
              label="Currency"
              onChange={(e) => setCurrencyFilter(e.target.value)}
              onOpen={() => refreshLookups()}
            >
              <MenuItem value="all">All</MenuItem>
              {currencies.map((c) => (
                <MenuItem key={c.id} value={c.id}>{currencyLabel(c)}</MenuItem>
              ))}
            </Select>
          </FormControl>
        )}
        <Button variant="contained" startIcon={<AddIcon />} onClick={openAddDialog}>
          Add {itemName}
        </Button>
      </Box>

      {loading ? (
        <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
          <CircularProgress />
        </Box>
      ) : (
        <ItemsTable
          rows={visibleRows}
          columns={columns}
          onRowUpdate={updateRow}
          onDelete={setDeleteId}
          onError={setError}
        />
      )}

      <AddItemDialog
        open={addOpen}
        title={`Add ${itemName}`}
        fields={fields}
        values={formValues}
        onChange={setFormValues}
        onSubmit={addItem}
        onClose={() => setAddOpen(false)}
      />

      <ConfirmDialog
        open={deleteId !== null}
        title="Delete item?"
        message="This cannot be undone."
        onConfirm={() => deleteRow(deleteId)}
        onCancel={() => setDeleteId(null)}
      />
    </Box>
  );
}
