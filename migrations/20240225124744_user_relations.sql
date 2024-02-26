CREATE OR REPLACE FUNCTION id_in_users() 
RETURNS int LANGUAGE SQL AS $$
   SELECT id FROM users LIMIT 1;
$$;

ALTER TABLE books ADD COLUMN user_id int NOT NULL DEFAULT id_in_users();
ALTER TABLE books ADD CONSTRAINT books_user_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE books ALTER COLUMN user_id DROP DEFAULT;

ALTER TABLE exports ADD COLUMN user_id int NOT NULL DEFAULT id_in_users();
ALTER TABLE exports ADD CONSTRAINT exports_user_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE books ALTER COLUMN user_id DROP DEFAULT;

-- Clearing covers to add links to bucket after migration
UPDATE books set cover = null;
