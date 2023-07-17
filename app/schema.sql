CREATE TABLE IF NOT EXISTS books (
    id serial PRIMARY KEY,
    name varchar(100) NOT null UNIQUE,
    chapter_count int DEFAULT 0
);

CREATE TABLE IF NOT EXISTS chapters (
    id serial PRIMARY KEY,
    book_id int NOT null,
    name varchar(100) NOT null,
    content text NOT null,
    number_in_book int NOT null UNIQUE
);

CREATE UNIQUE INDEX chapters_book_number_key on chapters(book_id, number_in_book);

CREATE TABLE IF NOT EXISTS exports (
  id serial PRIMARY KEY,
  meta jsonb NOT null,
  created_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT null,
  processing_started_at timestamptz DEFAULT null,
  processed_at timestamptz DEFAULT null,
  sent bool DEFAULT false NOT null,
  error text DEFAULT null
);
