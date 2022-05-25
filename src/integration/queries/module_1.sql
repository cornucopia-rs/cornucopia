--! insert_book
INSERT INTO Book (title)
  VALUES (:book_name);

--! nightmare ?{data, datas}
SELECT
  *
FROM
  nightmare;

--! copies
SELECT * FROM copy;

--! insert_copy
INSERT INTO Copy (composite, domain)
  VALUES (:composite, :domain);
