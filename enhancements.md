GitHub Copilot Chat Assistant — focused, actionable enhancements for the data model and processes in gsp8181/my-budget-rs

Quick summary: your repo uses Rocket + Diesel with SQLite (from Cargo.toml) and has a separate CLI and PHP/JS client. The highest-value changes are: (1) tighten money storage & integrity, (2) formalize reservation/budget rules and precomputed snapshots for fast allowance calculation, (3) make imports idempotent and auditable, and (4) add migrations/backup/CI processes. Below are concrete model and process changes, prioritized and with small examples.

High priority — correctness, integrity, and money handling

• Store money as integer minor units (amount_cents or amount_minor) in DB, not floating or DB decimal:

• Rationale: deterministic sums, consistent indexes, easier aggregation, no DB-specific decimal surprises.

• Use rust_decimal in app logic if you need decimals, but persist cents as INTEGER.

• Enforce referential integrity and constraints:

• Add foreign keys for account_id, user_id, category_id.

• Add NOT NULL where appropriate and CHECK constraints for valid enum values (account types, transaction kinds).

• Use DB transactions for multi-step operations (create transaction + update snapshots + audit log).

• Use explicit created_at/updated_at timestamps everywhere.

Example (Postgres/SQLite-compatible) table skeletons

• transactions:

• id (PK), account_id FK, user_id FK, amount_cents INTEGER NOT NULL, currency TEXT, date DATE, category_id FK, payee TEXT, notes TEXT, imported_hash TEXT UNIQUE NULL, cleared BOOLEAN, created_at TIMESTAMP

• reservations:

• id, user_id FK, account_id nullable, amount_cents, frequency (daily/weekday/weekly), start_date, end_date, enabled, created_at

• budgets:

• id, user_id, category_id nullable, period (daily/weekly/monthly), amount_cents, start_date, end_date, active

• snapshots:

• id, account_id, date, balance_cents (materialized daily snapshot used to compute allowances)

Medium priority — weekend-reserve & allowance logic (domain model)

• Make the weekend-reserve an explicit first-class concept (reservations or budget_rules), not an ad-hoc flag.

• Reservation record: specify amount (or percent), recurrence (Mon–Thu), and target (account/category).

• At snapshot time, mark reserved amounts and compute available_daily_allowance = (balance - reserved_future_amounts) / remaining_days.

• Precompute daily snapshots / allowances nightly:

• A background job expands recurring transactions for the upcoming period, applies reservations, computes next_run for recurring, and stores precomputed allowances (or cache).

• This avoids doing heavy calculations on every API request.

Medium priority — imports, deduplication, auditability

• Add import tracking:

• imported_hash or imported_id for each imported transaction (hash of account+date+amount+payee).

• Unique constraint on imported_hash to prevent duplicates.

• A separate imports table that stores original file metadata (filename, uploader, parse_time, stats).

• Add an immutable audit_log table for all user actions (create/update/delete) and import events:

• audit_logs(id, user_id, action, resource_type, resource_id, payload JSON, created_at).

• Helps debugging and user restores.

Medium priority — recurring transactions and scheduling

• Model recurring_transactions:

• Fields: id, user_id, account_id, amount_cents, currency, category_id, rrule/text schedule, next_run, enabled.

• Background worker expands recurring transactions to transactions table when next_run <= now and updates next_run.

• Support editing occurrences vs series (store original_recurring_id on generated transaction).

Performance & scaling considerations

• Indexes:

• transactions(user_id, date), transactions(account_id, date), transactions(category_id), snapshots(account_id, date).

• Materialize heavy queries:

• Maintain a materialized summary table for monthly/category totals to serve reports quickly.

• Concurrency:

• SQLite: fine for single-user/dev, but be aware of write limitations. For multi-user or concurrent writes, recommend Postgres.

• Use SELECT FOR UPDATE or optimistic locking (version column) for critical updates.

Migrations, backups, and environment strategy

• Use diesel_migrations (already in Cargo.toml) and keep migrations/ committed. Ensure app can run migrations on startup or CI.

• Local dev: support SQLite with seed data and dev migrations. Production: recommend Postgres (provide migration path and scripts).

• Backups:

• SQLite: document file backup approach (atomic copy), provide scheduled backup instructions (cron -> S3).

• Postgres: recommend managed backups (pg_dump, PITR).

• Version your schema and fail fast if the app sees an old schema version.

Testing, CI, and reproducible processes

• Add unit tests for allowance/reservation arithmetic and edge cases (e.g., negative balances, timezone boundary dates).

• Integration tests using an in-memory SQLite and optional dockerized Postgres to run API flows: import -> dedupe -> snapshot -> allowance computation.

• CI: run cargo fmt, cargo clippy, cargo test on each PR. Add a job to run client tests (npm test) if client included.

API design and UX improvements (process)

• Versioned API endpoints (v1).

• Support bulk endpoints for imports and batch transaction creation.

• Provide a read endpoint for precomputed allowance: GET /v1/allowance?date=2026-03-29 which returns {available_cents, reserved_cents, next_reservations}.

• Provide idempotent POSTs: accept client-generated idempotency key for upload/import operations.

Observability, logging & metrics

• Structured logs (JSON) with context (user_id, request_id).

• Metrics for snapshot job duration, number of recurring transactions expanded, import success/failure.

• Health endpoints (/health, /metrics) to integrate with container orchestration.

Data retention, soft deletes & GDPR-ish considerations

• Soft delete transactions (deleted boolean + deleted_at) to allow undo/restore and keep audit trail.

• Provide export tools for a user to download their data (JSON/CSV) and a way to purge per privacy requirements.

Advanced/optional ideas (longer term)

• Event-sourcing or append-only events for all changes, with projections for current state — good for audit, undo, replay but adds complexity.

• Multi-currency support with exchange rate table for historical conversion (if cross-currency accounts needed).

• Machine-learning style categorization: learn category matches for imported transactions via simple heuristics or a small ML model stored locally.

• Offer a bulk “what-if” calculator for planned reservations/transactions to show future balances.

Concrete prioritized roadmap (3–6 steps)

1. Immediate (safety & correctness)

• Add amount_cents storage + migrate any existing decimal usage in code.

• Add foreign keys, migrations, created_at/updated_at.

• Ensure imports generate a stable import hash and dedupe exists.

2. Short term (week)

• Implement reservations table and a nightly snapshot job to compute allowances (and an endpoint to read them).

• Add audit_logs and soft delete.

• Add unit tests for allowance logic.

3. Medium (month)

• Add recurring transaction expansion worker, scheduled tasks, and backups.

• Add CI for fmt/clippy/tests and a migration-on-start option.

• Provide a docker-compose/Devcontainer setup for local dev with example seed data.

4. Long term

• Migrate to Postgres if multi-user/concurrency expected.

• Add metrics, advanced reporting, and export/import features.

If you want, I can:

• Draft the SQL migration files (diesel-compatible) for transactions/reservations/snapshots/audit_logs.

• Create the nightly job scaffold (Rust + cron integration or a small worker binary).

• Write the API endpoints for allowance and import dedupe and the unit tests for the allowance logic.

Which of those concrete items should I prepare for a PR first? (e.g., “create migrations + models for transactions/reservations + unit tests”)