-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS Users (
  id SERIAL PRIMARY KEY,
  username varchar(100) NOT NULL
)