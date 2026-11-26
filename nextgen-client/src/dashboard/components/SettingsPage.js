import { useCallback, useEffect, useState } from 'react';
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
import Autocomplete from '@mui/material/Autocomplete';
import CurrenciesSection from './CurrenciesSection';
import API_BASE from '../../config';

export default function SettingsPage() {
  const [settings, setSettings] = useState({
    payday: '',
    weekdaySaving: '',
    dailyRate: '',
    pay: '',
    calc_to_eom: false,
    calc_following_month: false,
    timezone: '',
  });
  const [tzInfo, setTzInfo] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [savedMsg, setSavedMsg] = useState(false);

  // Whether the stored settings have been read; saving before then would overwrite
  // them with blank values (e.g. if the page was opened while offline).
  const [settingsLoaded, setSettingsLoaded] = useState(false);

  const fetchSettings = useCallback(async () => {
    try {
      const res = await fetch(`${API_BASE}/api/settings`);
      if (!res.ok) throw new Error(`Failed to fetch settings (${res.status})`);
      const data = await res.json();
      const mapped = {};
      for (const el of data) {
        mapped[el.name] = (el.name === 'calc_to_eom' || el.name === 'calc_following_month') ? el.value === 'true' : el.value;
      }
      setSettings(prev => ({ ...prev, ...mapped }));
      setSettingsLoaded(true);
      setError(null);
    } catch (e) {
      setError(`${e.message}. Retrying when the connection is back…`);
    }
  }, []);

  const fetchTimezones = useCallback(async () => {
    try {
      const res = await fetch(`${API_BASE}/api/timezone`);
      if (!res.ok) throw new Error(`Failed to fetch timezones (${res.status})`);
      const tz = await res.json();
      setTzInfo(tz);
      setSettings(prev => ({ ...prev, timezone: prev.timezone || tz.configured }));
    } catch (e) {
      // Shown as the dropdown's empty text; retried on open / reconnect.
    }
  }, []);

  useEffect(() => {
    Promise.all([fetchSettings(), fetchTimezones()]).finally(() => setLoading(false));
  }, [fetchSettings, fetchTimezones]);

  // Retry whatever failed once the browser is back online.
  useEffect(() => {
    const onOnline = () => {
      if (!settingsLoaded) fetchSettings();
      if (!tzInfo) fetchTimezones();
    };
    window.addEventListener('online', onOnline);
    // The browser can be online while the server isn't reachable, so also poll.
    const timer = (!settingsLoaded || !tzInfo) ? setInterval(onOnline, 5000) : null;
    return () => {
      window.removeEventListener('online', onOnline);
      if (timer) clearInterval(timer);
    };
  }, [settingsLoaded, tzInfo, fetchSettings, fetchTimezones]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!settingsLoaded) return;
    const payload = Object.keys(settings)
      .filter(key => !(key === 'timezone' && !settings.timezone))
      .map(key => ({
      name: key,
      value: String(settings[key]),
    }));
    try {
      const res = await fetch(`${API_BASE}/api/settings`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) {
        const detail = await res.text();
        throw new Error(detail || `Failed to save settings (${res.status})`);
      }
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
        <Box>
          <FormControlLabel
            control={
              <Checkbox
                checked={!!settings.calc_following_month}
                onChange={e => setSettings(p => ({ ...p, calc_following_month: e.target.checked }))}
              />
            }
            label="Include Payments Beyond Next Payday"
          />
          <Typography variant="caption" color="text.secondary" display="block" sx={{ ml: 4 }}>
            Also count recurring payments or credits that fall after the next payday but within the same month (e.g. paid on the 24th but have a direct debit on the 26th).
          </Typography>
        </Box>
        <Autocomplete
          options={tzInfo?.available ?? []}
          onOpen={() => { if (!tzInfo) fetchTimezones(); }}
          noOptionsText={tzInfo ? 'No matching timezone' : "Couldn't load timezones - retrying…"}
          value={settings.timezone || null}
          onChange={(_, value) => setSettings(p => ({ ...p, timezone: value ?? '' }))}
          disableClearable={!!settings.timezone}
          autoHighlight
          renderInput={(params) => (
            <TextField
              {...params}
              label="Timezone"
              helperText={
                tzInfo
                  ? `Used to decide what "today" is. System timezone: ${
                      tzInfo.system ?? 'unknown'
                    }${tzInfo.system_valid ? '' : ' (not recognised, defaulting to Europe/London)'}.`
                  : 'Used to decide what "today" is.'
              }
            />
          )}
          fullWidth
        />
        <Button type="submit" variant="contained" disabled={!settingsLoaded} sx={{ alignSelf: 'flex-start' }}>
          Save Settings
        </Button>
        <Divider />
        <CurrenciesSection />
      </Stack>
    </Box>
  );
}
