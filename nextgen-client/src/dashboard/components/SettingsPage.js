import { useEffect, useState } from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import CircularProgress from '@mui/material/CircularProgress';
import FormControlLabel from '@mui/material/FormControlLabel';
import Checkbox from '@mui/material/Checkbox';
import Stack from '@mui/material/Stack';
import Alert from '@mui/material/Alert';
import Divider from '@mui/material/Divider';
import CurrenciesSection from './CurrenciesSection';
import API_BASE from '../../config';

export default function SettingsPage() {
  const [settings, setSettings] = useState({
    payday: '',
    weekdaySaving: '',
    dailyRate: '',
    pay: '',
    calc_to_eom: false,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [savedMsg, setSavedMsg] = useState(false);

  useEffect(() => {
    const fetchSettings = async () => {
      try {
        const res = await fetch(`${API_BASE}/api/settings`);
        if (!res.ok) throw new Error(`Failed to fetch settings (${res.status})`);
        const data = await res.json();
        const mapped = {};
        for (const el of data) {
          mapped[el.name] = el.name === 'calc_to_eom' ? el.value === 'true' : el.value;
        }
        setSettings(prev => ({ ...prev, ...mapped }));
      } catch (e) {
        setError(e.message);
      } finally {
        setLoading(false);
      }
    };
    fetchSettings();
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    const payload = Object.keys(settings).map(key => ({
      name: key,
      value: String(settings[key]),
    }));
    try {
      const res = await fetch(`${API_BASE}/api/settings`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) throw new Error(`Failed to save settings (${res.status})`);
      setSavedMsg(true);
      setTimeout(() => setSavedMsg(false), 3000);
    } catch (e) {
      setError(e.message);
    }
  };

  if (loading) return <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}><CircularProgress /></Box>;

  return (
    <Box component="form" onSubmit={handleSubmit} sx={{ maxWidth: 480, width: '100%' }}>
      <Stack spacing={2}>
        {error && (
          <Alert severity="error" onClose={() => setError(null)}>{error}</Alert>
        )}
        {savedMsg && (
          <Alert severity="success">Settings saved successfully.</Alert>
        )}
        <TextField
          label="Payday"
          type="number"
          value={settings.payday}
          onChange={e => setSettings(p => ({ ...p, payday: e.target.value }))}
          inputProps={{ min: 1, max: 31 }}
          helperText="The day of the month you are paid monthly."
          fullWidth
        />
        <TextField
          label="Weekday Saving"
          type="number"
          value={settings.weekdaySaving}
          onChange={e => setSettings(p => ({ ...p, weekdaySaving: e.target.value }))}
          inputProps={{ step: '0.01' }}
          helperText="The amount you wish to save Monday–Thursday for the weekend."
          fullWidth
        />
        <TextField
          label="Daily Rate"
          type="number"
          value={settings.dailyRate}
          onChange={e => setSettings(p => ({ ...p, dailyRate: e.target.value }))}
          inputProps={{ step: '0.01' }}
          helperText="The total amount you wish to be made available per day."
          fullWidth
        />
        <TextField
          label="Total Pay"
          type="number"
          value={settings.pay}
          onChange={e => setSettings(p => ({ ...p, pay: e.target.value }))}
          inputProps={{ step: '0.01' }}
          helperText="The total net amount you get paid each month."
          fullWidth
        />
        <Box>
          <FormControlLabel
            control={
              <Checkbox
                checked={!!settings.calc_to_eom}
                onChange={e => setSettings(p => ({ ...p, calc_to_eom: e.target.checked }))}
              />
            }
            label="Calculate Daily Rate to End of Month"
          />
          <Typography variant="caption" color="text.secondary" display="block" sx={{ ml: 4 }}>
            If after payday, should the daily rate calculation terminate on the 1st of the end of the month.
          </Typography>
        </Box>
        <Button type="submit" variant="contained" sx={{ alignSelf: 'flex-start' }}>
          Save Settings
        </Button>
        <Divider />
        <CurrenciesSection />
      </Stack>
    </Box>
  );
}
