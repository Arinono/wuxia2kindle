-- Add migration script here
ALTER TABLE chapters DROP CONSTRAINT chapters_book_fkey;
ALTER TABLE chapters ADD CONSTRAINT chapters_book_fkey FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE;
