import { useEffect, useState, useCallback, useMemo } from 'react';
import { flushSync } from 'react-dom';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import AddIcon from '@mui/icons-material/Add';
import DeleteIcon from '@mui/icons-material/DeleteOutlined';
import CheckIcon from '@mui/icons-material/Check';
import CloseIcon from '@mui/icons-material/Close';
import { DataGrid, GridActionsCellItem, GridRowModes } from '@mui/x-data-grid';
import CircularProgress from '@mui/material/CircularProgress';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import DialogActions from '@mui/material/DialogActions';
import TextField from '@mui/material/TextField';
import MenuItem from '@mui/material/MenuItem';
import Select from '@mui/material/Select';
import FormControl from '@mui/material/FormControl';
import InputLabel from '@mui/material/InputLabel';
import Alert from '@mui/material/Alert';
import API_BASE from '../../config';

export default function CategoryPage({ apiPath, pageName, columnDefs, defaultFormValues, formFields, currencies, cards }) {
  const [rows, setRows] = useState([]);
  const [rowModesModel, setRowModesModel] = useState({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [addOpen, setAddOpen] = useState(false);
  const [formValues, setFormValues] = useState(defaultFormValues);
  const [currencyFilter, setCurrencyFilter] = useState('all');
  const [deleteId, setDeleteId] = useState(null);

  // Build a map of id -> currency for fast lookup
  const currencyMap = useMemo(() => {
    const map = {};
    (currencies || []).forEach(c => { map[c.id] = c; });
    return map;
  }, [currencies]);

  // Build a map of id -> card for fast lookup
  const cardMap = useMemo(() => {
    const map = {};
    (cards || []).forEach(c => { map[c.id] = c; });
    return map;
  }, [cards]);

  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_BASE}/api/${apiPath}`);
      if (!res.ok) throw new Error(`Failed to fetch data (${res.status})`);
      const data = await res.json();
      setRows(data);
    } catch (e) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  }, [apiPath]);

  useEffect(() => { fetchData(); }, [fetchData]);

  const handleCellClick = useCallback((params) => {
    if (params.field === 'actions') return;
    // flushSync makes the state update (and the re-render that creates the input element)
    // happen synchronously, still within the original touch/click event handler.
    // This keeps us inside the browser's user-gesture window so the mobile
    // keyboard opens immediately without needing a second tap.
    flushSync(() => {
      setRowModesModel((prev) => ({
        ...prev,
        [params.id]: { mode: GridRowModes.Edit, fieldToFocus: params.field },
      }));
    });
    const input = document.querySelector(
      `[data-id="${params.id}"] [data-field="${params.field}"] input,` +
      `[data-id="${params.id}"] [data-field="${params.field}"] textarea`
    );
    if (input) input.focus();
  }, []);

  const handleSave = useCallback((id) => {
    setRowModesModel((prev) => ({ ...prev, [id]: { mode: GridRowModes.View } }));
  }, []);

  const handleCancel = useCallback((id) => {
    setRowModesModel((prev) => ({
      ...prev,
      [id]: { mode: GridRowModes.View, ignoreModifications: true },
    }));
  }, []);

  const processRowUpdate = useCallback(async (newRow, oldRow) => {
    const changed = {};
    for (const key of Object.keys(newRow)) {
      if (key !== 'id' && newRow[key] !== oldRow[key]) changed[key] = newRow[key];
    }
    if (Object.keys(changed).length === 0) return oldRow;
    try {
      const res = await fetch(`${API_BASE}/api/${apiPath}/${newRow.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(changed),
      });
      if (!res.ok) throw new Error(`Failed to save (${res.status})`);
      return newRow;
    } catch (e) {
      setError(e.message);
      return oldRow;
    }
  }, [apiPath]);

  const handleDelete = useCallback(async (id) => {
    try {
      const res = await fetch(`${API_BASE}/api/${apiPath}/${id}`, { method: 'DELETE' });
      if (!res.ok) throw new Error(`Delete failed (${res.status})`);
      setRows((prev) => prev.filter((r) => r.id !== id));
    } catch (e) {
      setError(e.message);
    } finally {
      setDeleteId(null);
    }
  }, [apiPath]);

  const handleAdd = async () => {
    try {
      const res = await fetch(`${API_BASE}/api/${apiPath}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formValues),
      });
      if (!res.ok) throw new Error(`Failed to add item (${res.status})`);
      setAddOpen(false);
      setFormValues(defaultFormValues);
      fetchData();
    } catch (e) {
      setError(e.message);
    }
  };

  const handleAddKeyDown = (e) => {
    if (e.key === 'Enter') handleAdd();
  };

  // True when any row is in edit mode — used to show the currency column only while editing.
  const isAnyEditing = useMemo(
    () => Object.values(rowModesModel).some(m => m.mode === GridRowModes.Edit),
    [rowModesModel]
  );

  // Custom edit cell for currency_id — compact select showing only the symbol.
  const CurrencyEditCell = useCallback((params) => {
    return (
      <Select
        value={params.value ?? ''}
        onChange={(e) => params.api.setEditCellValue({ id: params.id, field: 'currency_id', value: e.target.value })}
        size="small"
        variant="standard"
        disableUnderline
        sx={{ width: '100%' }}
      >
        {(currencies || []).map(c => (
          <MenuItem key={c.id} value={c.id}>{c.symbol}</MenuItem>
        ))}
      </Select>
    );
  }, [currencies]);

  // Custom edit cell for cardid.
  const CardEditCell = useCallback((params) => {
    return (
      <Select
        value={params.value ?? ''}
        onChange={(e) => params.api.setEditCellValue({ id: params.id, field: 'cardid', value: e.target.value })}
        size="small"
        variant="standard"
        disableUnderline
        sx={{ width: '100%' }}
      >
        {(cards || []).map(c => (
          <MenuItem key={c.id} value={c.id}>{c.name}</MenuItem>
        ))}
      </Select>
    );
  }, [cards]);

  // Transform columnDefs: replace valueFormatter on 'amount' with a renderCell
  // that prepends the correct currency symbol from the row's currency_id.
  // Also inject an editable Currency column.
  const enhancedColumnDefs = useMemo(() => {
    const base = columnDefs.map((col) => {
      if (col.field === 'amount') {
        return {
          ...col,
          editable: true,
          valueFormatter: undefined,
          renderCell: (params) => {
            const currency = currencyMap[params.row.currency_id];
            const symbol = currency ? currency.symbol : '£';
            return `${symbol}${params.value}`;
          },
        };
      }
      if (col.isCard) {
        return {
          ...col,
          editable: true,
          renderCell: (params) => {
            const card = cardMap[params.value];
            return card ? card.name : params.value;
          },
          renderEditCell: CardEditCell,
        };
      }
      return { ...col, editable: true };
    });

    const currencyCol = {
      field: 'currency_id',
      headerName: 'Curr',
      width: 60,
      editable: true,
      renderCell: (params) => {
        const currency = currencyMap[params.value];
        return currency ? currency.symbol : '£';
      },
      renderEditCell: CurrencyEditCell,
    };

    return [...base, currencyCol];
  }, [columnDefs, currencyMap, cardMap, CurrencyEditCell, CardEditCell]);

  const columns = [
    ...enhancedColumnDefs,
    {
      field: 'actions',
      type: 'actions',
      headerName: '',
      width: 80,
      cellClassName: 'actions',
      getActions: ({ id }) => {
        const isEditing = rowModesModel[id]?.mode === GridRowModes.Edit;
        if (isEditing) {
          return [
            <GridActionsCellItem
              key="save"
              icon={<CheckIcon />}
              label="Save"
              onClick={() => handleSave(id)}
              color="success"
            />,
            <GridActionsCellItem
              key="cancel"
              icon={<CloseIcon />}
              label="Cancel"
              onClick={() => handleCancel(id)}
              color="error"
            />,
          ];
        }
        return [
          <GridActionsCellItem
            key="delete"
            icon={<DeleteIcon />}
            label="Delete"
            onClick={() => setDeleteId(id)}
            color="error"
          />,
        ];
      },
    },
  ];

  const filteredRows = currencyFilter === 'all'
    ? rows
    : rows.filter(r => String(r.currency_id) === String(currencyFilter));

  const hasCurrencies = (currencies || []).length > 1;

  return (
    <Box sx={{ width: '100%' }}>
      {error && (
        <Alert severity="error" onClose={() => setError(null)} sx={{ mb: 2 }}>
          {error}
        </Alert>
      )}
      <Box sx={{ mb: 2, display: 'flex', justifyContent: 'flex-end', alignItems: 'center', gap: 2 }}>
        {hasCurrencies && (
          <FormControl size="small" sx={{ minWidth: 140 }}>
            <InputLabel>Currency</InputLabel>
            <Select
              value={currencyFilter}
              label="Currency"
              onChange={e => setCurrencyFilter(e.target.value)}
            >
              <MenuItem value="all">All</MenuItem>
              {(currencies || []).map(c => (
                <MenuItem key={c.id} value={c.id}>{c.symbol} — {c.name}</MenuItem>
              ))}
            </Select>
          </FormControl>
        )}
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => { setFormValues(defaultFormValues); setAddOpen(true); }}
        >
          Add {pageName}
        </Button>
      </Box>
      {loading ? (
        <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
          <CircularProgress />
        </Box>
      ) : (
        <Box sx={{ width: '100%', overflowX: 'auto' }}>
          <DataGrid
            autoHeight
            editMode="row"
            rows={filteredRows}
            columns={columns}
            columnVisibilityModel={{ currency_id: isAnyEditing }}
            rowModesModel={rowModesModel}
            onRowModesModelChange={setRowModesModel}
            onCellClick={handleCellClick}
            processRowUpdate={processRowUpdate}
            onProcessRowUpdateError={(e) => setError(e.message)}
            pageSizeOptions={[10, 25, 50]}
            initialState={{ pagination: { paginationModel: { pageSize: 25 } } }}
            density="compact"
            disableColumnResize
            isCellEditable={() => true}
          />
        </Box>
      )}
      <Dialog open={addOpen} onClose={() => setAddOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Add {pageName}</DialogTitle>
        <DialogContent>
          {formFields.map((field, idx) => {
            if (field.type === 'currency') {
              return (
                <TextField
                  key={field.name}
                  select
                  label={field.label || 'Currency'}
                  value={formValues[field.name] ?? ''}
                  onChange={e => setFormValues(prev => ({ ...prev, [field.name]: e.target.value }))}
                  fullWidth
                  margin="normal"
                  autoFocus={idx === 0}
                >
                  {(currencies || []).map(c => (
                    <MenuItem key={c.id} value={c.id}>{c.symbol} — {c.name}</MenuItem>
                  ))}
                </TextField>
              );
            }
            if (field.type === 'card') {
              return (
                <TextField
                  key={field.name}
                  select
                  label={field.label || 'Card'}
                  value={formValues[field.name] ?? ''}
                  onChange={e => setFormValues(prev => ({ ...prev, [field.name]: e.target.value }))}
                  fullWidth
                  margin="normal"
                  autoFocus={idx === 0}
                >
                  {(cards || []).map(c => (
                    <MenuItem key={c.id} value={c.id}>{c.name}</MenuItem>
                  ))}
                </TextField>
              );
            }
            return (
              <TextField
                key={field.name}
                label={field.label}
                type={field.type || 'text'}
                value={formValues[field.name] ?? ''}
                onChange={e => setFormValues(prev => ({ ...prev, [field.name]: e.target.value }))}
                onKeyDown={idx === formFields.length - 1 ? handleAddKeyDown : undefined}
                fullWidth
                margin="normal"
                inputProps={field.inputProps}
                placeholder={field.placeholder}
                autoFocus={idx === 0}
              />
            );
          })}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setAddOpen(false)}>Cancel</Button>
          <Button variant="contained" onClick={handleAdd}>Add</Button>
        </DialogActions>
      </Dialog>

      <Dialog open={deleteId !== null} onClose={() => setDeleteId(null)}>
        <DialogTitle>Delete item?</DialogTitle>
        <DialogContent>This cannot be undone.</DialogContent>
        <DialogActions>
          <Button onClick={() => setDeleteId(null)}>Cancel</Button>
          <Button variant="contained" color="error" onClick={() => handleDelete(deleteId)}>Delete</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
