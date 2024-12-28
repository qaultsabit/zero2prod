CREATE TABLE subscriptions(
    id uuid PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    subsribed_at TIMESTAMPTZ NOT NULL
);
