--! insert_book (author?)
INSERT INTO book (author, name) VALUES (:author, :name);

--! select_book: (author?)
SELECT * FROM book;

--! find_books: (author?)
SELECT * FROM book WHERE name = ANY (:title);

--! params_use_twice
UPDATE book SET name = :name WHERE length(name) > 42 AND length(:name) < 42;

--! params_order
UPDATE imaginary SET c=:c, a=:a, z=:a, r=:c;