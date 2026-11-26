import API_BASE from './config';

// Small wrapper around fetch for the budget API.
// - Paths are relative to /api, e.g. api.get('/settings').
// - JSON bodies are encoded for you; JSON responses are decoded.
// - Any non-2xx response throws an Error whose message is the server's error
//   text if it sent one (e.g. "Unknown timezone 'X'"), otherwise a generic one.

async function request(method, path, body) {
  const options = { method };
  if (body !== undefined) {
    options.headers = { 'Content-Type': 'application/json' };
    options.body = JSON.stringify(body);
  }

  const res = await fetch(`${API_BASE.replace(/\/$/, '')}/api${path}`, options);

  if (!res.ok) {
    const detail = await res.text().catch(() => '');
    throw new Error(detail || `${method} /api${path} failed (${res.status})`);
  }

  const contentType = res.headers.get('content-type') || '';
  return contentType.includes('application/json') ? res.json() : res.text();
}

const api = {
  get: (path) => request('GET', path),
  post: (path, body) => request('POST', path, body),
  put: (path, body) => request('PUT', path, body),
  delete: (path) => request('DELETE', path),
};

export default api;
