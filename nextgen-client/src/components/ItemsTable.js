import { useCallback, useMemo, useState } from 'react';
import Box from '@mui/material/Box';
import Select from '@mui/material/Select';
import CheckIcon from '@mui/icons-material/Check';
import CloseIcon from '@mui/icons-material/Close';
import DeleteIcon from '@mui/icons-material/DeleteOutlined';
import { DataGrid, GridActionsCellItem, GridRowModes } from '@mui/x-data-grid';
import { useLookups } from '../LookupsContext';
import { cardLabel, currencySymbol, lookupOptions } from './lookupOptions';

// Editable table of budget items. Click a cell to edit its row; the tick saves
// (via onRowUpdate) and the cross cancels. Amounts show their currency symbol,
// card columns show the card's name, and a currency column appears while editing.
//
//   columns      DataGrid columns from pageConfig.js (a column with isCard: true is a card picker)
//   onRowUpdate  async (newRow, oldRow) => row to keep; throw to reject the edit
//   onDelete     (id) => void, called when a row's delete button is clicked
//   onError      (message) => void
export default function ItemsTable({ rows, columns, onRowUpdate, onDelete, onError }) {
  const { currencies, cards, status, refresh } = useLookups();
  const [rowModesModel, setRowModesModel] = useState({});

  const currencyById = useMemo(() => Object.fromEntries(currencies.map((c) => [c.id, c])), [currencies]);
  const cardById = useMemo(() => Object.fromEntries(cards.map((c) => [c.id, c])), [cards]);

  // Dropdown used when editing the currency or card of a row.
  const lookupEditCell = useCallback((field, items, what, label) => (params) => (
    <Select
      value={params.value ?? ''}
      onChange={(e) => params.api.setEditCellValue({ id: params.id, field, value: e.target.value })}
      onOpen={() => refresh()}
      size="small"
      variant="standard"
      disableUnderline
      sx={{ width: '100%' }}
    >
      {lookupOptions(items, { what, status, label })}
    </Select>
  ), [refresh, status]);

  const startEditing = useCallback((params) => {
    if (params.field === 'actions') return;
    setRowModesModel((prev) => ({
      ...prev,
      [params.id]: { mode: GridRowModes.Edit, fieldToFocus: params.field },
    }));
  }, []);

  const setRowMode = (id, mode) => setRowModesModel((prev) => ({ ...prev, [id]: mode }));

  const isAnyEditing = Object.values(rowModesModel).some((m) => m.mode === GridRowModes.Edit);

  const gridColumns = useMemo(() => {
    const dataColumns = columns.map((col) => {
      if (col.field === 'amount') {
        return {
          ...col,
          editable: true,
          renderCell: ({ row, value }) => `${currencyById[row.currency_id]?.symbol ?? '£'}${value}`,
        };
      }
      if (col.isCard) {
        return {
          ...col,
          editable: true,
          renderCell: ({ value }) => cardById[value]?.name ?? value,
          renderEditCell: lookupEditCell('cardid', cards, 'cards', cardLabel),
        };
      }
      return { ...col, editable: true };
    });

    const currencyColumn = {
      field: 'currency_id',
      headerName: 'Curr',
      width: 60,
      editable: true,
      renderCell: ({ value }) => currencyById[value]?.symbol ?? '£',
      renderEditCell: lookupEditCell('currency_id', currencies, 'currencies', currencySymbol),
    };

    const actionsColumn = {
      field: 'actions',
      type: 'actions',
      headerName: '',
      width: 80,
      cellClassName: 'actions',
      getActions: ({ id }) => (rowModesModel[id]?.mode === GridRowModes.Edit
        ? [
          <GridActionsCellItem key="save" icon={<CheckIcon />} label="Save" color="success"
            onClick={() => setRowMode(id, { mode: GridRowModes.View })} />,
          <GridActionsCellItem key="cancel" icon={<CloseIcon />} label="Cancel" color="error"
            onClick={() => setRowMode(id, { mode: GridRowModes.View, ignoreModifications: true })} />,
        ]
        : [
          <GridActionsCellItem key="delete" icon={<DeleteIcon />} label="Delete" color="error"
            onClick={() => onDelete(id)} />,
        ]),
    };

    return [...dataColumns, currencyColumn, actionsColumn];
  }, [columns, currencies, cards, currencyById, cardById, lookupEditCell, rowModesModel, onDelete]);

  return (
    <Box sx={{ width: '100%', overflowX: 'auto' }}>
      <DataGrid
        autoHeight
        editMode="row"
        rows={rows}
        columns={gridColumns}
        columnVisibilityModel={{ currency_id: isAnyEditing }}
        rowModesModel={rowModesModel}
        onRowModesModelChange={setRowModesModel}
        onCellClick={startEditing}
        processRowUpdate={onRowUpdate}
        onProcessRowUpdateError={(e) => onError(e.message)}
        pageSizeOptions={[10, 25, 50]}
        initialState={{ pagination: { paginationModel: { pageSize: 25 } } }}
        density="compact"
        disableColumnResize
        isCellEditable={() => true}
      />
    </Box>
  );
}
