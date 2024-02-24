-- Add migration script here
ALTER TABLE chapters ADD CONSTRAINT chapters_book_fkey FOREIGN KEY (book_id) REFERENCES books(id);
