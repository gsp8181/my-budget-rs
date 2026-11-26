// Single list of every page in the app. The sidebar (MenuContent) and the
// router (Dashboard) are both built from this, so a page only needs adding here.
//
// Category pages are all rendered by CategoryPage:
//   path      URL of the page
//   menuLabel text in the sidebar
//   icon      sidebar icon component
//   section   'credits' or 'debits' - which sidebar group it sits in
//   title     heading shown at the top of the page
//   apiPath   backend endpoint under /api/
//   itemName  used for the "Add <itemName>" button and dialog
//   columns   DataGrid column definitions for the table
//   fields    inputs in the "Add" dialog (type 'currency' / 'card' render dropdowns;
//             'person' is free text with suggestions of people already entered)
//
// Note on the debt pages: /api/debt is stored as a credit (money owed to me)
// and /api/debtto as a debit (money I owe).

import HomeRoundedIcon from '@mui/icons-material/HomeRounded';
import AccountBalanceIcon from '@mui/icons-material/AccountBalance';
import MoneyIcon from '@mui/icons-material/Money';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import CreditCardIcon from '@mui/icons-material/CreditCard';
import HourglassEmptyIcon from '@mui/icons-material/HourglassEmpty';
import PeopleRoundedIcon from '@mui/icons-material/PeopleRounded';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import PaymentIcon from '@mui/icons-material/Payment';
import RemoveCircleOutlineIcon from '@mui/icons-material/RemoveCircleOutline';
import TrendingDownIcon from '@mui/icons-material/TrendingDown';
import SettingsRoundedIcon from '@mui/icons-material/SettingsRounded';

// ---- Reusable table columns ----
const nameCol = (headerName, extra = {}) => ({ field: 'name', headerName, flex: 1, ...extra });
const amountCol = { field: 'amount', headerName: 'Amount', width: 90 };
const dayCol = (headerName = 'Day') => ({ field: 'day', headerName, width: 55 });
const cardCol = { field: 'cardid', headerName: 'Card', width: 100, isCard: true };
const personCol = (headerName) => ({ field: 'person', headerName, flex: 1, minWidth: 100, isPerson: true });

// ---- Reusable form fields ----
const nameField = (label, placeholder) => ({ name: 'name', label, placeholder });
const amountField = { name: 'amount', label: 'Amount', type: 'number', inputProps: { step: '0.01' } };
const dayField = { name: 'day', label: 'Day of Month', type: 'number', inputProps: { min: 1, max: 31 } };
const cardField = { name: 'cardid', label: 'Card Used', type: 'card' };
const currencyField = { name: 'currency_id', label: 'Currency', type: 'currency' };
const personField = (label, placeholder) => ({ name: 'person', label, placeholder, type: 'person' });

export const homePage = { path: '/', menuLabel: 'Home', icon: HomeRoundedIcon, title: 'Home' };
export const settingsPage = { path: '/settings', menuLabel: 'Settings', icon: SettingsRoundedIcon, title: 'Settings' };

export const categoryPages = [
  // ---- Credits ----
  {
    path: '/bank',
    menuLabel: 'Bank',
    icon: AccountBalanceIcon,
    section: 'credits',
    title: 'Bank Accounts',
    apiPath: 'bank',
    itemName: 'Bank Account',
    columns: [nameCol('Account'), amountCol],
    fields: [nameField('Account Name', 'Main Current Account'), amountField, currencyField],
  },
  {
    path: '/cash',
    menuLabel: 'Cash',
    icon: MoneyIcon,
    section: 'credits',
    title: 'Cash',
    apiPath: 'cash',
    itemName: 'Cash Entry',
    columns: [nameCol('Description'), amountCol],
    fields: [nameField('Description', 'Change in Wallet'), amountField, currencyField],
  },
  {
    path: '/regularcredit',
    menuLabel: 'Regular Credit',
    icon: TrendingUpIcon,
    section: 'credits',
    title: 'Regular Credit',
    apiPath: 'regularcredit',
    itemName: 'Regular Credit',
    columns: [nameCol('Creditor', { minWidth: 100 }), amountCol, dayCol()],
    fields: [nameField('Creditor', 'Rental Income'), amountField, dayField, currencyField],
  },
  {
    path: '/cardheld',
    menuLabel: 'Card Items Held',
    icon: CreditCardIcon,
    section: 'credits',
    title: 'Card Items Held Off Balance',
    apiPath: 'cardheld',
    itemName: 'Card Item',
    columns: [nameCol('Item', { minWidth: 100 }), amountCol, cardCol],
    fields: [nameField('Item', 'Business Expense'), amountField, cardField, currencyField],
  },
  {
    path: '/uncleared',
    menuLabel: 'Uncleared Items',
    icon: HourglassEmptyIcon,
    section: 'credits',
    title: 'Uncleared Items',
    apiPath: 'uncleared',
    itemName: 'Uncleared Item',
    columns: [nameCol('Item', { minWidth: 100 }), amountCol, cardCol],
    fields: [nameField('Item', 'Offline Card Payment'), amountField, cardField, currencyField],
  },
  {
    path: '/debt',
    menuLabel: 'Debt Owed To Me',
    icon: PeopleRoundedIcon,
    section: 'credits',
    title: 'Debt Owed To Me',
    apiPath: 'debt',
    itemName: 'Debt',
    columns: [personCol('Debtor'), nameCol('Description', { minWidth: 100 }), amountCol],
    fields: [personField('Debtor (optional)', 'John Doe'), nameField('Description', 'Pizza'), amountField, currencyField],
  },
  {
    path: '/misccredit',
    menuLabel: 'Misc Credit',
    icon: AddCircleOutlineIcon,
    section: 'credits',
    title: 'Miscellaneous Credit',
    apiPath: 'misccredit',
    itemName: 'Misc Credit',
    columns: [nameCol('Description'), amountCol],
    fields: [nameField('Description', '...'), amountField, currencyField],
  },

  // ---- Debits ----
  {
    path: '/cardbalance',
    menuLabel: 'Card Balance',
    icon: CreditCardIcon,
    section: 'debits',
    title: 'Card Balance',
    apiPath: 'cardbalance',
    itemName: 'Card Balance',
    columns: [nameCol('Card Name'), amountCol],
    fields: [nameField('Card Name', 'Main Credit Card'), amountField, currencyField],
  },
  {
    path: '/regularpayment',
    menuLabel: 'Regular Payment',
    icon: PaymentIcon,
    section: 'debits',
    title: 'Regular Payment',
    apiPath: 'regularpayment',
    itemName: 'Regular Payment',
    columns: [nameCol('Description', { minWidth: 100 }), amountCol, dayCol('Day Taken')],
    fields: [nameField('Description', 'Phone Bill'), amountField, dayField, currencyField],
  },
  {
    path: '/debtto',
    menuLabel: 'Debt I Owe',
    icon: TrendingDownIcon,
    section: 'debits',
    title: 'Debt I Owe',
    apiPath: 'debtto',
    itemName: 'Debt',
    columns: [personCol('Debtee'), nameCol('Description', { minWidth: 100 }), amountCol],
    fields: [personField('Debtee (optional)', 'John Doe'), nameField('Description', 'Pizza'), amountField, currencyField],
  },
  {
    path: '/miscdebit',
    menuLabel: 'Misc Debit',
    icon: RemoveCircleOutlineIcon,
    section: 'debits',
    title: 'Miscellaneous Debit',
    apiPath: 'miscdebit',
    itemName: 'Misc Debit',
    columns: [nameCol('Description'), amountCol],
    fields: [nameField('Description', '...'), amountField, currencyField],
  },
];

export const creditPages = categoryPages.filter((p) => p.section === 'credits');
export const debitPages = categoryPages.filter((p) => p.section === 'debits');
