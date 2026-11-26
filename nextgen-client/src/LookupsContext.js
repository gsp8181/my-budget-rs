import { createContext, useCallback, useContext, useEffect, useRef, useState } from 'react';
import api from './api';

// Shared dropdown data (currencies, cards and the people named on debts), with refresh-on-demand so dropdowns
// recover from a failed first load and pick up items added elsewhere in the app.

const RETRY_MS = 5000;

const LookupsContext = createContext({
  currencies: [],
  cards: [],
  people: [],
  status: 'loading',
  refresh: () => Promise.resolve(),
});

export function LookupsProvider({ children }) {
  const [currencies, setCurrencies] = useState([]);
  const [cards, setCards] = useState([]);
  const [people, setPeople] = useState([]);
  // 'loading' until the first successful load, then 'ok'; 'error' while the latest attempt failed.
  const [status, setStatus] = useState('loading');
  const inflight = useRef(null);

  const refresh = useCallback(() => {
    // Collapse concurrent refreshes (e.g. focus + dropdown open) into one request pair.
    if (inflight.current) return inflight.current;
    const request = (async () => {
      try {
        const [cur, crd, ppl] = await Promise.all([
          api.get('/currency'), api.get('/cardbalance'), api.get('/people'),
        ]);
        setCurrencies(cur);
        setCards(crd);
        setPeople(ppl);
        setStatus('ok');
      } catch {
        // Keep whatever lists we already had; just flag the failure.
        setStatus('error');
      } finally {
        inflight.current = null;
      }
    })();
    inflight.current = request;
    return request;
  }, []);

  useEffect(() => {
    refresh();
    window.addEventListener('online', refresh);
    window.addEventListener('focus', refresh);
    return () => {
      window.removeEventListener('online', refresh);
      window.removeEventListener('focus', refresh);
    };
  }, [refresh]);

  // Keep retrying in the background while the last load failed.
  useEffect(() => {
    if (status !== 'error') return undefined;
    const timer = setInterval(refresh, RETRY_MS);
    return () => clearInterval(timer);
  }, [status, refresh]);

  return (
    <LookupsContext.Provider value={{ currencies, cards, people, status, refresh }}>
      {children}
    </LookupsContext.Provider>
  );
}

export function useLookups() {
  return useContext(LookupsContext);
}
