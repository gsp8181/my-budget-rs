import * as React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import { useEffect, useState } from 'react';

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
import API_BASE from '../config';

const xThemeComponents = {
  ...chartsCustomizations,
  ...dataGridCustomizations,
  ...datePickersCustomizations,
  ...treeViewCustomizations,
};

const currencyField = { name: 'currency_id', label: 'Currency', type: 'currency' };

const amountCol = { field: 'amount', headerName: 'Amount', width: 90 };
const cardCol = { field: 'cardid', headerName: 'Card', width: 100, isCard: true };
const dayCol = (label = 'Day') => ({ field: 'day', headerName: label, width: 55 });

const categoryRoutes = [
  {
    path: '/bank',
    pageName: 'Bank Accounts',
    apiPath: 'bank',
    itemName: 'Bank Account',
    columnDefs: [
      { field: 'name', headerName: 'Account', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Account Name', placeholder: 'Main Current Account' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
  {
    path: '/cash',
    pageName: 'Cash',
    apiPath: 'cash',
    itemName: 'Cash Entry',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: 'Change in Wallet' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
  {
    path: '/regularcredit',
    pageName: 'Regular Credit',
    apiPath: 'regularcredit',
    itemName: 'Regular Credit',
    columnDefs: [
      { field: 'name', headerName: 'Creditor', flex: 1, minWidth: 100 },
      amountCol,
      dayCol(),
    ],
    defaultFormValues: { name: '', amount: '', day: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Creditor', placeholder: 'Rental Income' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'day', label: 'Day of Month', type: 'number', inputProps: { min: 1, max: 31 } },
      currencyField,
    ],
  },
  {
    path: '/cardheld',
    pageName: 'Card Items Held Off Balance',
    apiPath: 'cardheld',
    itemName: 'Card Item',
    useCards: true,
    columnDefs: [
      { field: 'name', headerName: 'Item', flex: 1, minWidth: 100 },
      amountCol,
      cardCol,
    ],
    defaultFormValues: { name: '', amount: '', cardid: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Item', placeholder: 'Business Expense' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'cardid', label: 'Card Used', type: 'card' },
      currencyField,
    ],
  },
  {
    path: '/uncleared',
    pageName: 'Uncleared Items',
    apiPath: 'uncleared',
    itemName: 'Uncleared Item',
    useCards: true,
    columnDefs: [
      { field: 'name', headerName: 'Item', flex: 1, minWidth: 100 },
      amountCol,
      cardCol,
    ],
    defaultFormValues: { name: '', amount: '', cardid: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Item', placeholder: 'Offline Card Payment' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'cardid', label: 'Card Used', type: 'card' },
      currencyField,
    ],
  },
  {
    path: '/debtto',
    pageName: 'Debt Owed To Me',
    apiPath: 'debtto',
    itemName: 'Debt',
    columnDefs: [
      { field: 'name', headerName: 'Debtee', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Debtee', placeholder: 'Owed for Pizza' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
  {
    path: '/misccredit',
    pageName: 'Miscellaneous Credit',
    apiPath: 'misccredit',
    itemName: 'Misc Credit',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: '...' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
  {
    path: '/cardbalance',
    pageName: 'Card Balance',
    apiPath: 'cardbalance',
    itemName: 'Card Balance',
    columnDefs: [
      { field: 'name', headerName: 'Card Name', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Card Name', placeholder: 'Main Credit Card' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
  {
    path: '/regularpayment',
    pageName: 'Regular Payment',
    apiPath: 'regularpayment',
    itemName: 'Regular Payment',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1, minWidth: 100 },
      amountCol,
      dayCol('Day Taken'),
    ],
    defaultFormValues: { name: '', amount: '', day: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: 'Phone Bill' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      { name: 'day', label: 'Day of Month', type: 'number', inputProps: { min: 1, max: 31 } },
      currencyField,
    ],
  },
  {
    path: '/debt',
    pageName: 'Debt I Owe',
    apiPath: 'debt',
    itemName: 'Debt',
    columnDefs: [
      { field: 'name', headerName: 'Debtor', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Debtor', placeholder: 'John Doe' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
  {
    path: '/miscdebit',
    pageName: 'Miscellaneous Debit',
    apiPath: 'miscdebit',
    itemName: 'Misc Debit',
    columnDefs: [
      { field: 'name', headerName: 'Description', flex: 1 },
      amountCol,
    ],
    defaultFormValues: { name: '', amount: '', currency_id: '' },
    formFields: [
      { name: 'name', label: 'Description', placeholder: '...' },
      { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } },
      currencyField,
    ],
  },
];

export default function Dashboard(props) {
  const [currencies, setCurrencies] = useState([]);
  const [cards, setCards] = useState([]);

  useEffect(() => {
    fetch(`${API_BASE}/api/currency`)
      .then(r => r.ok ? r.json() : [])
      .then(data => setCurrencies(data))
      .catch(() => {});
    fetch(`${API_BASE}/api/cardbalance`)
      .then(r => r.ok ? r.json() : [])
      .then(data => setCards(data))
      .catch(() => {});
  }, []);

  // Default to the oldest currency (lowest id) which is always £.
  const defaultCurrencyId = currencies.length > 0 ? currencies[0].id : '';

  return (
    <AppTheme {...props} themeComponents={xThemeComponents}>
      <CssBaseline enableColorScheme />
      <Router>
        <Box sx={{ display: 'flex', height: '100dvh', overflow: 'hidden' }}>
          <SideMenu />
          <AppNavbar />
          {/* Main content */}
          <Box
            component="main"
            sx={(theme) => ({
              flexGrow: 1,
              height: '100%',
              backgroundColor: theme.vars
                ? `rgba(${theme.vars.palette.background.defaultChannel} / 1)`
                : alpha(theme.palette.background.default, 1),
              overflowY: 'auto',
            })}
          >
            <Stack
              spacing={2}
              sx={{
                alignItems: 'center',
                mx: { xs: 0, md: 3 },
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
                {categoryRoutes.map(({ path, pageName, apiPath, itemName, columnDefs, defaultFormValues, formFields, useCards }) => (
                  <Route key={path} path={path} element={
                    <>
                      <Header pageName={pageName} />
                      <CategoryPage
                        apiPath={apiPath}
                        pageName={itemName}
                        columnDefs={columnDefs}
                        defaultFormValues={{ ...defaultFormValues, currency_id: defaultCurrencyId }}
                        formFields={formFields}
                        currencies={currencies}
                        cards={useCards ? cards : []}
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
