-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    email VARCHAR(255) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    display_name VARCHAR(500),
    image TEXT,
    role VARCHAR(255) NOT NULL,
    description TEXT,
    token TEXT
);
