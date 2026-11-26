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
import { creditPages, debitPages, homePage, settingsPage } from '../pageConfig';

export default function MenuContent({ onNavigate }) {
  const navigate = useNavigate();
  const location = useLocation();

  const isSelected = (path) =>
    path === '/' ? location.pathname === '/' : location.pathname === path;

  const renderItem = ({ path, menuLabel, icon: Icon }) => (
    <ListItem key={path} disablePadding sx={{ display: 'block' }}>
      <ListItemButton selected={isSelected(path)} onClick={() => { navigate(path); onNavigate?.(); }}>
        <ListItemIcon><Icon /></ListItemIcon>
        <ListItemText primary={menuLabel} />
      </ListItemButton>
    </ListItem>
  );

  return (
    <Stack sx={{ flexGrow: 1, p: 1, justifyContent: 'space-between' }}>
      <List dense>
        {renderItem(homePage)}
        <Divider sx={{ my: 1 }} />
        <Typography variant="caption" sx={{ px: 1.5, py: 0.5, color: 'text.secondary', display: 'block' }}>
          Credits
        </Typography>
        {creditPages.map(renderItem)}
        <Divider sx={{ my: 1 }} />
        <Typography variant="caption" sx={{ px: 1.5, py: 0.5, color: 'text.secondary', display: 'block' }}>
          Debits
        </Typography>
        {debitPages.map(renderItem)}
      </List>

      <List dense>
        {renderItem(settingsPage)}
      </List>
    </Stack>
  );
}
