pub mod module_1 {
use deadpool_postgres::{Client, Transaction};
use tokio_postgres::{types::Type, error::Error};


pub async fn insert_book_one(client:&Client, ) -> Result<(),Error> {let stmt = client.prepare_typed_cached("INSERT INTO Book (title)
VALUES ('bob');
", &[]).await?;
let res = client.execute(&stmt, &[]).await?;

Ok(())}
pub async fn insert_book_one_tx<'a>(client:&Transaction<'a>, ) -> Result<(), Error> {let stmt = client.prepare_typed_cached("INSERT INTO Book (title)
VALUES ('bob');
", &[]).await?;
let res = client.execute(&stmt, &[]).await?;

Ok(())}



pub async fn insert_book_zero_or_one(client:&Client, ) -> Result<(),Error> {let stmt = client.prepare_typed_cached("INSERT INTO Book (title)
VALUES ('alice');
", &[]).await?;
let res = client.execute(&stmt, &[]).await?;

Ok(())}
pub async fn insert_book_zero_or_one_tx<'a>(client:&Transaction<'a>, ) -> Result<(), Error> {let stmt = client.prepare_typed_cached("INSERT INTO Book (title)
VALUES ('alice');
", &[]).await?;
let res = client.execute(&stmt, &[]).await?;

Ok(())}



pub async fn insert_book_zero_or_more(client:&Client, ) -> Result<(),Error> {let stmt = client.prepare_typed_cached("INSERT INTO Book (title)
VALUES ('carl');
", &[]).await?;
let res = client.execute(&stmt, &[]).await?;

Ok(())}
pub async fn insert_book_zero_or_more_tx<'a>(client:&Transaction<'a>, ) -> Result<(), Error> {let stmt = client.prepare_typed_cached("INSERT INTO Book (title)
VALUES ('carl');
", &[]).await?;
let res = client.execute(&stmt, &[]).await?;

Ok(())}

}

pub mod module_2 {
use deadpool_postgres::{Client, Transaction};
use tokio_postgres::{types::Type, error::Error};


pub async fn authors(client:&Client, ) -> Result<Vec<(i32,String,String)>,Error> {let stmt = client.prepare_typed_cached("SELECT
*
FROM
Author;
", &[]).await?;
let res = client.query(&stmt, &[]).await?;

let return_value = res.iter().map(|res| { let return_value_0: i32 = res.get(0); let return_value_1: String = res.get(1); let return_value_2: String = res.get(2); (return_value_0,return_value_1,return_value_2) }).collect::<Vec<(i32,String,String)>>(); Ok(return_value)}
pub async fn authors_tx<'a>(client:&Transaction<'a>, ) -> Result<Vec<(i32,String,String)>, Error> {let stmt = client.prepare_typed_cached("SELECT
*
FROM
Author;
", &[]).await?;
let res = client.query(&stmt, &[]).await?;

let return_value = res.iter().map(|res| { let return_value_0: i32 = res.get(0); let return_value_1: String = res.get(1); let return_value_2: String = res.get(2); (return_value_0,return_value_1,return_value_2) }).collect::<Vec<(i32,String,String)>>(); Ok(return_value)}


#[derive(Debug, Clone, PartialEq)]
pub struct Books {pub title : String}
pub async fn books(client:&Client, ) -> Result<Vec<Books>,Error> {let stmt = client.prepare_typed_cached("SELECT
Title
FROM
Book;
", &[]).await?;
let res = client.query(&stmt, &[]).await?;

let return_value = res.iter().map(|res| { let return_value_0: String = res.get(0); Books { title: return_value_0 } }).collect::<Vec<Books>>(); Ok(return_value)}
pub async fn books_tx<'a>(client:&Transaction<'a>, ) -> Result<Vec<Books>, Error> {let stmt = client.prepare_typed_cached("SELECT
Title
FROM
Book;
", &[]).await?;
let res = client.query(&stmt, &[]).await?;

let return_value = res.iter().map(|res| { let return_value_0: String = res.get(0); Books { title: return_value_0 } }).collect::<Vec<Books>>(); Ok(return_value)}



pub async fn books_from_author_id(client:&Client, id : &i32) -> Result<Vec<String>,Error> {let stmt = client.prepare_typed_cached("SELECT
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Id = $1;
", &[]).await?;
let res = client.query(&stmt, &[&id]).await?;

let return_value = res.iter().map(|row| {let value : String = row.get(0); value}).collect::<Vec<String>>(); Ok(return_value)}
pub async fn books_from_author_id_tx<'a>(client:&Transaction<'a>, id : &i32) -> Result<Vec<String>, Error> {let stmt = client.prepare_typed_cached("SELECT
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Id = $1;
", &[]).await?;
let res = client.query(&stmt, &[&id]).await?;

let return_value = res.iter().map(|row| {let value : String = row.get(0); value}).collect::<Vec<String>>(); Ok(return_value)}



pub async fn author_name_by_id_opt(client:&Client, id : &i32) -> Result<Option<String>,Error> {let stmt = client.prepare_typed_cached("SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
", &[]).await?;
let res = client.query_opt(&stmt, &[&id]).await?;

let return_value = res.map(|row| {let value: String = row.get(0); value}); Ok(return_value)}
pub async fn author_name_by_id_opt_tx<'a>(client:&Transaction<'a>, id : &i32) -> Result<Option<String>, Error> {let stmt = client.prepare_typed_cached("SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
", &[]).await?;
let res = client.query_opt(&stmt, &[&id]).await?;

let return_value = res.map(|row| {let value: String = row.get(0); value}); Ok(return_value)}



pub async fn author_name_by_id(client:&Client, id : &i32) -> Result<String,Error> {let stmt = client.prepare_typed_cached("SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
", &[]).await?;
let res = client.query_one(&stmt, &[&id]).await?;

let return_value: String = res.get(0); Ok(return_value)}
pub async fn author_name_by_id_tx<'a>(client:&Transaction<'a>, id : &i32) -> Result<String, Error> {let stmt = client.prepare_typed_cached("SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
", &[]).await?;
let res = client.query_one(&stmt, &[&id]).await?;

let return_value: String = res.get(0); Ok(return_value)}



pub async fn author_name_starting_with(client:&Client, s : &str) -> Result<Vec<(i32,String,i32,String)>,Error> {let stmt = client.prepare_typed_cached("SELECT
BookAuthor.AuthorId,
Author.Name,
BookAuthor.BookId,
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Name LIKE CONCAT($1, '%');
", &[Type::TEXT]).await?;
let res = client.query(&stmt, &[&s]).await?;

let return_value = res.iter().map(|res| { let return_value_0: i32 = res.get(0); let return_value_1: String = res.get(1); let return_value_2: i32 = res.get(2); let return_value_3: String = res.get(3); (return_value_0,return_value_1,return_value_2,return_value_3) }).collect::<Vec<(i32,String,i32,String)>>(); Ok(return_value)}
pub async fn author_name_starting_with_tx<'a>(client:&Transaction<'a>, s : &str) -> Result<Vec<(i32,String,i32,String)>, Error> {let stmt = client.prepare_typed_cached("SELECT
BookAuthor.AuthorId,
Author.Name,
BookAuthor.BookId,
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Name LIKE CONCAT($1, '%');
", &[Type::TEXT]).await?;
let res = client.query(&stmt, &[&s]).await?;

let return_value = res.iter().map(|res| { let return_value_0: i32 = res.get(0); let return_value_1: String = res.get(1); let return_value_2: i32 = res.get(2); let return_value_3: String = res.get(3); (return_value_0,return_value_1,return_value_2,return_value_3) }).collect::<Vec<(i32,String,i32,String)>>(); Ok(return_value)}

}