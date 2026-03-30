import * as React from 'react';
import { DataGrid } from '@mui/x-data-grid';
import formatCurrency from '../../utils/formatCurrency';

const columns = [
  { field: 'id', headerName: 'ID', width: 90 },
  { field: 'category', headerName: 'Category', width: 150 },
  { field: 'name', headerName: 'Name', width: 150 },
  {
    field: 'amount',
    headerName: 'Amount',
    width: 130,
    type: 'number',
    valueFormatter: (value) => formatCurrency(value),
  },
  { field: 'dbName', headerName: 'Database Name', width: 150 },
];

export default function StaticDataGrid({ data }) {
  return (
    <DataGrid
      autoHeight
      checkboxSelection
      rows={data}
      columns={columns}
      getRowClassName={(params) =>
        params.indexRelativeToCurrentPage % 2 === 0 ? 'even' : 'odd'
      }
      initialState={{
        pagination: { paginationModel: { pageSize: 20 } },
      }}
      pageSizeOptions={[10, 20, 50]}
      disableColumnResize
      density="compact"
      slotProps={{
        filterPanel: {
          filterFormProps: {
            logicOperatorInputProps: {
              variant: 'outlined',
              size: 'small',
            },
            columnInputProps: {
              variant: 'outlined',
              size: 'small',
              sx: { mt: 'auto' },
            },
            operatorInputProps: {
              variant: 'outlined',
              size: 'small',
              sx: { mt: 'auto' },
            },
            valueInputProps: {
              InputComponentProps: {
                variant: 'outlined',
                size: 'small',
              },
            },
          },
        },
      }}
    />
  );
}
