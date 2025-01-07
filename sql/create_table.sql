CREATE TABLE IF NOT EXISTS transaction (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    amount FLOAT NOT NULL,
    category TEXT NOT NULL,
    date TEXT NOT NULL
);
