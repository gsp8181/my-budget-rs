import { useEffect, useState, useCallback } from 'react';
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
import Alert from '@mui/material/Alert';
import API_BASE from '../../config';

export default function CategoryPage({ apiPath, pageName, columnDefs, defaultFormValues, formFields }) {
  const [rows, setRows] = useState([]);
  const [rowModesModel, setRowModesModel] = useState({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [addOpen, setAddOpen] = useState(false);
  const [formValues, setFormValues] = useState(defaultFormValues);

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
    // Don't enter edit mode when clicking the actions column
    if (params.field === 'actions') return;
    setRowModesModel((prev) => ({
      ...prev,
      [params.id]: { mode: GridRowModes.Edit, fieldToFocus: params.field },
    }));
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
    // Build only the changed fields to PUT
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

  const editableColumnDefs = columnDefs.map((col) => ({ ...col, editable: true }));

  const columns = [
    ...editableColumnDefs,
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
            onClick={() => handleDelete(id)}
            color="error"
          />,
        ];
      },
    },
  ];

  return (
    <Box sx={{ width: '100%' }}>
      {error && (
        <Alert severity="error" onClose={() => setError(null)} sx={{ mb: 2 }}>
          {error}
        </Alert>
      )}
      <Box sx={{ mb: 2, display: 'flex', justifyContent: 'flex-end' }}>
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
        <DataGrid
          autoHeight
          editMode="row"
          rows={rows}
          columns={columns}
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
      )}
      <Dialog open={addOpen} onClose={() => setAddOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Add {pageName}</DialogTitle>
        <DialogContent>
          {formFields.map((field, idx) => (
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
          ))}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setAddOpen(false)}>Cancel</Button>
          <Button variant="contained" onClick={handleAdd}>Add</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
