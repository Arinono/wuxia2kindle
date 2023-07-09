CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    chapter_count int2 DEFAULT 0
);

CREATE TABLE IF NOT EXISTS chapters (
    id SERIAL PRIMARY KEY,
    book_id int NOT NULL,
    name VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    number_in_book int2 NOT NULL,
    processed bool DEFAULT false NOT NULL,
    processed_at timestamptz DEFAULT NULL
);

CREATE UNIQUE INDEX chapters_book_number_key on chapters(book_id, number_in_book);
