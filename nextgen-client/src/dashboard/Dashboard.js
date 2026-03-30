import * as React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import { alpha } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';
import Stack from '@mui/material/Stack';
import AppNavbar from './components/AppNavbar';
import Header from './components/Header';
import MainGrid from './components/MainGrid';
import SideMenu from './components/SideMenu';
import CategoryPage from './components/CategoryPage';
import SettingsPage from './components/SettingsPage';
import AppTheme from '../shared-theme/AppTheme';
import {
  chartsCustomizations,
  dataGridCustomizations,
  datePickersCustomizations,
  treeViewCustomizations,
} from './theme/customizations';

const xThemeComponents = {
  ...chartsCustomizations,
  ...dataGridCustomizations,
  ...datePickersCustomizations,
  ...treeViewCustomizations,
};

const categoryRoutes = [
  {
    path: '/bank',
    pageName: 'Bank Accounts',
    apiPath: 'bank',
    itemName: 'Bank Account',
    columnDefs: [
      { field: 'name', headerName: 'Account', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Account Name', placeholder: 'Main Current Account' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
  {
    path: '/cash',
    pageName: 'Cash',
    apiPath: 'cash',
    itemName: 'Cash Entry',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: 'Change in Wallet' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
  {
    path: '/regularcredit',
    pageName: 'Regular Credit',
    apiPath: 'regularcredit',
    itemName: 'Regular Credit',
    columnDefs: [
      { field: 'name', headerName: 'Creditor', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
      { field: 'day', headerName: 'Day', width: 100 },
    ],
    defaultFormValues: { name: '', amount: '', day: '' },
    formFields: [
      { name: 'name', label: 'Creditor', placeholder: 'Rental Income' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'day', label: 'Day of Month', type: 'number', inputProps: { min: 1, max: 31 } },
    ],
  },
  {
    path: '/cardheld',
    pageName: 'Card Items Held Off Balance',
    apiPath: 'cardheld',
    itemName: 'Card Item',
    columnDefs: [
      { field: 'name', headerName: 'Item', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
      { field: 'cardid', headerName: 'Card Used', width: 120 },
    ],
    defaultFormValues: { name: '', amount: '', cardid: '' },
    formFields: [
      { name: 'name', label: 'Item', placeholder: 'Business Expense' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'cardid', label: 'Card ID', type: 'number' },
    ],
  },
  {
    path: '/uncleared',
    pageName: 'Uncleared Items',
    apiPath: 'uncleared',
    itemName: 'Uncleared Item',
    columnDefs: [
      { field: 'name', headerName: 'Item', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
      { field: 'cardid', headerName: 'Card Used', width: 120 },
    ],
    defaultFormValues: { name: '', amount: '', cardid: '' },
    formFields: [
      { name: 'name', label: 'Item', placeholder: 'Offline Card Payment' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'cardid', label: 'Card ID', type: 'number' },
    ],
  },
  {
    path: '/debtto',
    pageName: 'Debt Owed To Me',
    apiPath: 'debtto',
    itemName: 'Debt',
    columnDefs: [
      { field: 'name', headerName: 'Debtee', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Debtee', placeholder: 'Owed for Pizza' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
  {
    path: '/misccredit',
    pageName: 'Miscellaneous Credit',
    apiPath: 'misccredit',
    itemName: 'Misc Credit',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: '...' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
  {
    path: '/cardbalance',
    pageName: 'Card Balance',
    apiPath: 'cardbalance',
    itemName: 'Card Balance',
    columnDefs: [
      { field: 'name', headerName: 'Card Name', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Card Name', placeholder: 'Main Credit Card' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
  {
    path: '/regularpayment',
    pageName: 'Regular Payment',
    apiPath: 'regularpayment',
    itemName: 'Regular Payment',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
      { field: 'day', headerName: 'Day Taken', width: 120 },
    ],
    defaultFormValues: { name: '', amount: '', day: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: 'Phone Bill' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'day', label: 'Day of Month', type: 'number', inputProps: { min: 1, max: 31 } },
    ],
  },
  {
    path: '/debt',
    pageName: 'Debt I Owe',
    apiPath: 'debt',
    itemName: 'Debt',
    columnDefs: [
      { field: 'name', headerName: 'Debtor', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Debtor', placeholder: 'John Doe' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
  {
    path: '/miscdebit',
    pageName: 'Miscellaneous Debit',
    apiPath: 'miscdebit',
    itemName: 'Misc Debit',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      { field: 'amount', headerName: 'Amount', width: 150, valueFormatter: (v) => `£${v}` },
    ],
    defaultFormValues: { name: '', amount: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: '...' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
    ],
  },
];

export default function Dashboard(props) {

  return (
    <AppTheme {...props} themeComponents={xThemeComponents}>
      <CssBaseline enableColorScheme />
      <Router>
        <Box sx={{ display: 'flex' }}>
          <SideMenu />
          <AppNavbar />
          {/* Main content */}
          <Box
            component="main"
            sx={(theme) => ({
              flexGrow: 1,
              backgroundColor: theme.vars
                ? `rgba(${theme.vars.palette.background.defaultChannel} / 1)`
                : alpha(theme.palette.background.default, 1),
              overflow: 'auto',
            })}
          >
            <Stack
              spacing={2}
              sx={{
                alignItems: 'center',
                mx: 3,
                pb: 5,
                mt: { xs: 8, md: 0 },
              }}
            >
              <Routes>
                <Route path="/" element={
                  <>
                    <Header pageName="Home" />
                    <MainGrid />
                  </>
                } />
                {categoryRoutes.map(({ path, pageName, apiPath, itemName, columnDefs, defaultFormValues, formFields }) => (
                  <Route key={path} path={path} element={
                    <>
                      <Header pageName={pageName} />
                      <CategoryPage
                        apiPath={apiPath}
                        pageName={itemName}
                        columnDefs={columnDefs}
                        defaultFormValues={defaultFormValues}
                        formFields={formFields}
                      />
                    </>
                  } />
                ))}
                <Route path="/settings" element={
                  <>
                    <Header pageName="Settings" />
                    <SettingsPage />
                  </>
                } />
              </Routes>
            </Stack>
          </Box>
        </Box>
      </Router>
    </AppTheme>
  );
}
