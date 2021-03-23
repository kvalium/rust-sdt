CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4 (),
  first_name varchar NOT NULL,
  last_name varchar NOT NULL,
  email varchar NOT NULL,
  pin_code smallint NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

SELECT
  diesel_manage_updated_at ('users');

