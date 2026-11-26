# my-budget web client

React + Material UI front end for the my-budget Rust API.

## Development

```bash
npm install
npm start        # dev server on http://localhost:3000, talks to the API on http://localhost:5540
```

Run the Rust API alongside it (`cargo run` in the repo root; debug builds listen on port 5540).
Set `REACT_APP_API_URL` to point at a different API.

## Production build

```bash
npm run build    # outputs to build/
```

The Dockerfile builds this and serves `build/` from the Rust binary's `wwwroot`.
`../build-client.sh` does the same for a local `wwwroot`.
