-- Add migration script here
CREATE TABLE IF NOT EXISTS Todos (
  id SERIAL PRIMARY KEY,
  title varchar(100) NOT NULL,
  done boolean not null default false
);