--! insert_clone
INSERT INTO clone (composite) VALUES (:composite);

--! select_clone
SELECT * FROM clone;

--! insert_copy
INSERT INTO copy (composite) VALUES (:composite);

--! select_copy
SELECT * FROM copy;