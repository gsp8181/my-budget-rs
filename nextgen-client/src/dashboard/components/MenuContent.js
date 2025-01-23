import * as React from 'react';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Stack from '@mui/material/Stack';
import HomeRoundedIcon from '@mui/icons-material/HomeRounded';
import AnalyticsRoundedIcon from '@mui/icons-material/AnalyticsRounded';
import PeopleRoundedIcon from '@mui/icons-material/PeopleRounded';
import AssignmentRoundedIcon from '@mui/icons-material/AssignmentRounded';
import SettingsRoundedIcon from '@mui/icons-material/SettingsRounded';
import InfoRoundedIcon from '@mui/icons-material/InfoRounded';
import HelpRoundedIcon from '@mui/icons-material/HelpRounded';

const mainListItems = [
  { text: 'Home', icon: <HomeRoundedIcon /> },
  { text: '' },
  { text: 'Bank', icon: <AnalyticsRoundedIcon /> },
  { text: 'Cash', icon: <PeopleRoundedIcon /> },
  { text: 'Regular Credit', icon: <AssignmentRoundedIcon /> },
  { text: 'Card Items Held Off Balance', icon: <AssignmentRoundedIcon /> },
  { text: 'Uncleared Item', icon: <AssignmentRoundedIcon /> },
  { text: 'Debt Owed To Me', icon: <AssignmentRoundedIcon /> },
  { text: 'Misc Credit', icon: <AssignmentRoundedIcon /> },
  { text: '' },
  { text: 'Card Balance', icon: <AssignmentRoundedIcon /> },
  { text: 'Regular Payment', icon: <AssignmentRoundedIcon /> },
  { text: 'Debt I Owe', icon: <AssignmentRoundedIcon /> },
  { text: 'Misc Debit', icon: <AssignmentRoundedIcon /> },
];

const secondaryListItems = [
  { text: 'Settings', icon: <SettingsRoundedIcon /> },
];

export default function MenuContent() {
  return (
    <Stack sx={{ flexGrow: 1, p: 1, justifyContent: 'space-between' }}>
      <List dense>
        {mainListItems.map((item, index) => (
          <ListItem key={index} disablePadding sx={{ display: 'block' }}>
            <ListItemButton selected={index === 0}>
              <ListItemIcon>{item.icon}</ListItemIcon>
              <ListItemText primary={item.text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      <List dense>
        {secondaryListItems.map((item, index) => (
          <ListItem key={index} disablePadding sx={{ display: 'block' }}>
            <ListItemButton>
              <ListItemIcon>{item.icon}</ListItemIcon>
              <ListItemText primary={item.text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </Stack>
  );
}
