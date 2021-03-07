CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4 (),
  first_name varchar NOT NULL,
  last_name varchar NOT NULL,
  email varchar NOT NULL,
  -- pin_code smallint NOT NULL,
  -- created_at timestamptz DEFAULT NOW() NOT NULL,
  -- deleted_at timestamptz,
  PRIMARY KEY (id)
);

