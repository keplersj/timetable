-- Your SQL goes here
CREATE TABLE scheduled_requests
(
    id SERIAL PRIMARY KEY,
    hook TEXT NOT NULL,
    time timestamptz NOT NULL,
    executed BOOLEAN NOT NULL DEFAULT 'f'
)