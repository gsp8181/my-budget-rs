import * as React from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import Divider from '@mui/material/Divider';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';
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

const topItems = [
  { text: 'Home', icon: <HomeRoundedIcon />, path: '/' },
];

const creditItems = [
  { text: 'Bank', icon: <AccountBalanceIcon />, path: '/bank' },
  { text: 'Cash', icon: <MoneyIcon />, path: '/cash' },
  { text: 'Regular Credit', icon: <TrendingUpIcon />, path: '/regularcredit' },
  { text: 'Card Items Held', icon: <CreditCardIcon />, path: '/cardheld' },
  { text: 'Uncleared Items', icon: <HourglassEmptyIcon />, path: '/uncleared' },
  { text: 'Debt Owed To Me', icon: <PeopleRoundedIcon />, path: '/debt' },
  { text: 'Misc Credit', icon: <AddCircleOutlineIcon />, path: '/misccredit' },
];

const debitItems = [
  { text: 'Card Balance', icon: <CreditCardIcon />, path: '/cardbalance' },
  { text: 'Regular Payment', icon: <PaymentIcon />, path: '/regularpayment' },
  { text: 'Debt I Owe', icon: <TrendingDownIcon />, path: '/debtto' },
  { text: 'Misc Debit', icon: <RemoveCircleOutlineIcon />, path: '/miscdebit' },
];

const secondaryListItems = [
  { text: 'Settings', icon: <SettingsRoundedIcon />, path: '/settings' },
];

export default function MenuContent({ onNavigate }) {
  const navigate = useNavigate();
  const location = useLocation();

  const isSelected = (path) =>
    path === '/' ? location.pathname === '/' : location.pathname === path;

  const renderItem = (item) => (
    <ListItem key={item.path} disablePadding sx={{ display: 'block' }}>
      <ListItemButton selected={isSelected(item.path)} onClick={() => { navigate(item.path); onNavigate?.(); }}>
        <ListItemIcon>{item.icon}</ListItemIcon>
        <ListItemText primary={item.text} />
      </ListItemButton>
    </ListItem>
  );

  return (
    <Stack sx={{ flexGrow: 1, p: 1, justifyContent: 'space-between' }}>
      <List dense>
        {topItems.map(renderItem)}
        <Divider sx={{ my: 1 }} />
        <Typography variant="caption" sx={{ px: 1.5, py: 0.5, color: 'text.secondary', display: 'block' }}>
          Credits
        </Typography>
        {creditItems.map(renderItem)}
        <Divider sx={{ my: 1 }} />
        <Typography variant="caption" sx={{ px: 1.5, py: 0.5, color: 'text.secondary', display: 'block' }}>
          Debits
        </Typography>
        {debitItems.map(renderItem)}
      </List>

      <List dense>
        {secondaryListItems.map(renderItem)}
      </List>
    </Stack>
  );
}
