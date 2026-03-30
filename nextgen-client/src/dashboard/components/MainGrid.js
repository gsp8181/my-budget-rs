import * as React from 'react';
import { useEffect, useState } from 'react';
// eslint-disable-next-line import/no-unresolved
import Grid from '@mui/material/Grid2';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';
import CircularProgress from '@mui/material/CircularProgress';
import Alert from '@mui/material/Alert';
import StaticDataGrid from './StaticDataGrid';
import DumbCard from './DumbCard';
import API_BASE from '../../config';
import formatCurrency from '../../utils/formatCurrency';

export default function MainGrid() {
  const [data, setData] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetch(`${API_BASE}/api`)
      .then((res) => {
        if (!res.ok) throw new Error(`Failed to load dashboard (${res.status})`);
        return res.json();
      })
      .then(setData)
      .catch((e) => setError(e.message));
  }, []);

  if (error) {
    return <Alert severity="error">{error}</Alert>;
  }

  if (!data) {
    return (
      <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
        <CircularProgress />
      </Box>
    );
  }

  const overviewCards = [
    { title: 'Current Amount', value: formatCurrency(data.amount), description: 'Current balance', trend: 'neutral' },
    { title: 'End of Week', value: formatCurrency(data.end_of_week), description: 'Amount left at end of week', trend: 'neutral' },
    { title: 'Remaining Week', value: formatCurrency(data.remaining_week), description: 'Amount left for the rest of the week', trend: 'neutral' },
    { title: 'Full Weekend', value: formatCurrency(data.full_weekend), description: 'Amount left for the weekend', trend: 'neutral' },
  ];

  const summaryCards = [
    { title: 'Monthly Credits', value: formatCurrency(data.monthly_credits), description: 'Credits this month', trend: 'up' },
    { title: 'Monthly Debits', value: formatCurrency(data.monthly_debits), description: 'Debits this month', trend: 'down' },
    { title: 'Card Balance Held', value: formatCurrency(data.card_held_total), description: 'Amount held on card balances', trend: 'down' },
    { title: 'Avg Monthly Saved', value: formatCurrency(data.net_saved_avg), description: 'Average amount saved per month', trend: 'up' },
    { title: 'Saved This Year', value: formatCurrency(data.saved_this_year), description: 'Total saved this year', trend: 'up' },
  ];

  return (
    <Box sx={{ width: '100%', maxWidth: { sm: '100%', md: '1700px' } }}>
      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Overview
      </Typography>
      <Grid container spacing={2} sx={{ mb: (theme) => theme.spacing(2) }}>
        {overviewCards.map((card) => (
          <Grid key={card.title} xs={12} sm={6} lg={3}>
            <DumbCard {...card} />
          </Grid>
        ))}
      </Grid>

      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Today's Transactions
      </Typography>
      <Grid container spacing={2} sx={{ mb: (theme) => theme.spacing(2) }}>
        <Grid xs={12}>
          <Paper variant="outlined" sx={{ p: 2 }}>
            <StaticDataGrid data={data.today} />
          </Paper>
        </Grid>
      </Grid>

      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Monthly Summary
      </Typography>
      <Grid container spacing={2}>
        {summaryCards.map((card) => (
          <Grid key={card.title} xs={12} sm={6} lg={3}>
            <DumbCard {...card} />
          </Grid>
        ))}
      </Grid>
    </Box>
  );
}

