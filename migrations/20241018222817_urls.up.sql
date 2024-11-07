-- Add up migration script here
CREATE TABLE urls (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    creator uuid REFERENCES users(id) NOT NULL,
    title varchar(64) NOT NULL,
    description varchar(256),
    long_url varchar(2048) NOT NULL,
    short_url varchar(16) UNIQUE NOT NULL,
    times_visited integer DEFAULT 0 NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp DEFAULT now() NOT NULL
);