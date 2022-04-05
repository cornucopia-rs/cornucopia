--! book_authors()
--! {author_id, name, book_id, title} *
SELECT
  ba.AuthorId,
  a.Name AuthorName,
  ba.BookId,
  b.Title AS BookTitle
FROM
  BooksAuthors AS ba
  INNER JOIN Authors AS a ON a.id = ba.authorid
  INNER JOIN Books AS b ON b.id = ba.bookid;

--! very_custom(first_letter: CHAR, book_title)
--! {author_id, name, book_id, title}
SELECT
  ba.AuthorId,
  a.Name AS AuthorName,
  ba.BookId,
  b.Title AS BookTitle
FROM
  BooksAuthors AS ba
  INNER JOIN Authors AS a ON a.id = ba.authorid
  INNER JOIN Books AS b ON b.id = ba.bookid
WHERE
  a.Name LIKE CONCAT($1, '%')
  AND b.Title = $2;

