DROP SCHEMA PUBLIC CASCADE;
CREATE SCHEMA PUBLIC;

CREATE TABLE todo (
  id SERIAL PRIMARY KEY,
  task TEXT NOT NULL,
  started TEXT NOT NULL,
  completed TEXT NOT NULL
);