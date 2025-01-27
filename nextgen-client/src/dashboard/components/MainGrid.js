import * as React from 'react';
import Grid from '@mui/material/Grid2';
import Box from '@mui/material/Box';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';
import Copyright from '../internals/components/Copyright';
import ChartUserByCountry from './ChartUserByCountry';
import CustomizedTreeView from './CustomizedTreeView';
import CustomizedDataGrid from './CustomizedDataGrid';
import StaticDataGrid from './StaticDataGrid';
import HighlightedCard from './HighlightedCard';
import PageViewsBarChart from './PageViewsBarChart';
import SessionsChart from './SessionsChart';
import StatCard from './StatCard';
import DumbCard from './DumbCard';

const axios = require('axios').default;

// // Fetch data from the API
// axios.get('https://budget.gsp8181.co.uk/api')
//   .then(response => {
//     console.log(response.data);
//     // You can update the state or handle the response data here
//   })
//   .catch(error => {
//     console.error('There was an error fetching the data!', error);
//   });

var fakeapidata = {
  "amount": "-1.30",
  "remaining_week": "328.70",
  "end_of_week": "168.70",
  "full_weekend": "248.70",
  "monthly_debits": "836.40",
  "monthly_credits": "3489.79",
  "net_saved_this_month": "-1",
  "card_held_total": "3917.10",
  "net_saved_avg": "1103.39",
  "saved_this_year": "13578.18",
  "today": [{"id":1,"oldId":null,"category":"bank","name":"Starling","day":null,"amount":"907.02","cardid":null,"dbName":"credit"},{"id":2,"oldId":null,"category":"bank","name":"Chase","day":null,"amount":"0","cardid":null,"dbName":"credit"}]
}

const extractedData = [
  {
    title: "Amount",
    value: `£${fakeapidata.amount}`,
    description: "Amount spent today",
    trend: "up"
  },
  {
    title: "End of Week",
    value: `£${fakeapidata.end_of_week}`,
    description: "Amount left at the end of the week",
    trend: "neutral"
  },
  {
    title: "Remaining Week",
    value: `£${fakeapidata.remaining_week}`,
    description: "Amount left for the rest of the week",
        trend: "neutral"
  },
  {
    title: "Full Weekend",
    value: `£${fakeapidata.full_weekend}`,
    description: "Amount left for the weekend",
        trend: "neutral"
  }
];

const extractedData2 = [
  {
    title: "Monthly Credits",
    value: `£${fakeapidata.monthly_credits}`,
    description: "Credits monthly",
    trend: "up"
  },
  {
    title: "Monthly Debits",
    value: `£${fakeapidata.monthly_debits}`,
    description: "Debits Monthly",
    trend: "down"
  },
  {
    title: "Card Balance Held",
    value: `£${fakeapidata.card_held_total}`,
    description: "Amount saved from card balances",
        trend: "down"
  },
  {
    title: "Monthly Saved",
    value: `£${fakeapidata.net_saved_avg}`,
    description: "Amount saved per month",
        trend: "up"
  },
  {
    title: "Yearly Saved",
    value: `£${fakeapidata.saved_this_year}`,
    description: "Amount saved per annum",
        trend: "up"
  }
];

export default function MainGrid() {
  return (
    <Box sx={{ width: '100%', maxWidth: { sm: '100%', md: '1700px' } }}>
      {/* cards */}
      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Dashboard
      </Typography>
      <Grid
        container
        spacing={2}
        columns={12}
        sx={{ mb: (theme) => theme.spacing(2) }}
      >
        {extractedData.map((card, index) => (
          <Grid key={index} size={{ xs: 12, sm: 6, lg: 3 }}>
            <DumbCard {...card} />
          </Grid>
        ))}
      </Grid>
      <Typography component="h2" variant="h6" sx={{ mb: 2 }}>
        Details
      </Typography>
      <Grid container spacing={2} columns={12}>
        <Grid size={{ xs: 12, lg: 9 }}>
          <StaticDataGrid data={fakeapidata.today} />
        </Grid>
      </Grid>
      <Grid
        container
        spacing={2}
        columns={12}
        sx={{ mb: (theme) => theme.spacing(2) }}
      >
        {extractedData2.map((card, index) => (
          <Grid key={index} size={{ xs: 12, sm: 6, lg: 3 }}>
            <DumbCard {...card} />
          </Grid>
        ))}
      </Grid>
      <Copyright sx={{ my: 4 }} />
    </Box>
  );
}
