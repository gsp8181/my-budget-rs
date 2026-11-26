import * as React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import { alpha } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';
import Stack from '@mui/material/Stack';
import AppNavbar from './components/AppNavbar';
import Header from './components/Header';
import HomePage from './pages/HomePage';
import SideMenu from './components/SideMenu';
import CategoryPage from './pages/CategoryPage';
import SettingsPage from './pages/SettingsPage';
import { LookupsProvider } from './LookupsContext';
import { categoryPages, homePage, settingsPage } from './pageConfig';
import AppTheme from './theme/AppTheme';

export default function App(props) {
  return (
    <LookupsProvider>
    <AppTheme {...props}>
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
                <Route path={homePage.path} element={
                  <>
                    <Header pageName={homePage.title} />
                    <HomePage />
                  </>
                } />
                {categoryPages.map((page) => (
                  <Route key={page.path} path={page.path} element={
                    <>
                      <Header pageName={page.title} />
                      <CategoryPage
                        apiPath={page.apiPath}
                        pageName={page.itemName}
                        columnDefs={page.columns}
                        defaultFormValues={Object.fromEntries(page.fields.map((f) => [f.name, '']))}
                        formFields={page.fields}
                        useCards={page.fields.some((f) => f.type === 'card')}
                      />
                    </>
                  } />
                ))}
                <Route path={settingsPage.path} element={
                  <>
                    <Header pageName={settingsPage.title} />
                    <SettingsPage />
                  </>
                } />
              </Routes>
            </Stack>
          </Box>
        </Box>
      </Router>
    </AppTheme>
    </LookupsProvider>
  );
}
