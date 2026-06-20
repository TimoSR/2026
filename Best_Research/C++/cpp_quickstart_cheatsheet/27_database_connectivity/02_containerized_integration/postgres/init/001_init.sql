CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE
);

INSERT INTO users (email) VALUES ('seed@example.com')
ON CONFLICT (email) DO NOTHING;
