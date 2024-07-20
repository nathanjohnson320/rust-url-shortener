CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    long_url TEXT NOT NULL,
    short_url TEXT NOT NULL
);
CREATE UNIQUE INDEX urls_short_url_index ON urls (short_url);
CREATE INDEX urls_long_url_index ON urls (long_url);