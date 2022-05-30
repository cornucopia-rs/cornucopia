--! insert_book
INSERT INTO book (author, name) VALUES (:author, :name);

--! select_book
SELECT * FROM book;

--! params_use_twice
UPDATE book SET name = :name WHERE length(name) > 42 AND length(:name) < 42;