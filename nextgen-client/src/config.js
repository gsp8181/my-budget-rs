// In production, API calls go to the same origin as the app is served from.
// In development, the backend runs on a separate port.
const API_BASE =
  process.env.REACT_APP_API_URL ||
  (process.env.NODE_ENV === 'production'
    ? window.location.origin
    : 'http://localhost:5540');

export default API_BASE;
