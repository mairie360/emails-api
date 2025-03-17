-- Your SQL goes here
CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    sender VARCHAR(320) NOT NULL,
    recipient VARCHAR(320) NOT NULL,
    subject TEXT,
    body TEXT,
    is_read BOOLEAN DEFAULT FALSE,
    received_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
