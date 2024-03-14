
use crate::models::Book;
use std::error::Error;
use sqlx::Row;
use sqlx::FromRow;
use sqlx::query;

pub async fn create(book:&Book,pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
     let query = "INSERT INTO scans (title,author,isbn) VALUES ($1,$2,$3) ";

     sqlx::query(query)
         .bind(&book.title)
         .bind(&book.author)
         .bind(&book.isbn)
         .execute(pool).await?;

    Ok(())
}

//just little bit complicated way to create book
async fn  insert_transaction(
	book: Book, conn: &sqlx::PgPool
) -> Result<(),Box<dyn Error>>{
     let mut txn = conn.begin().await?;

     let author_q = r"
        INSERT INTO author (name) VALUES ($1) RETURNING id
     ";

     let book_q = r"
       INSERT INTO book (title,author_id,isbn)
       VALUES ($1,$2,$3)";

    let author_id: (i64,) = sqlx::query_as(author_q)
        .bind(&book.author)
        .fetch_one(&mut txn)
        .await?;
     sqlx::query(book_q)
     .bind(&book.title)
     .bind(author_id.0)
     .bind(&book.isbn)
     .execute(&mut txn).await?; 

     txn.commit().await?;
     Ok(())
} 


pub async fn update(book:&Book,pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
     let query = "UPDATE INTO scans (title,author,isbn) VALUES ($1,$2,$3) ";

     sqlx::query(query)
         .bind(&book.title)
         .bind(&book.author)
         .bind(&book.isbn)
         .execute(pool).await?;

    Ok(())
}

pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>>{
	let q = "SELECT * FROM scans";
	let query = sqlx::query_as::<_,Book>(q);

    let books = query.fetch_all(conn).await?;
	/*while let Some(row) = rows.try_next().await? {
		books.push(Book {
		title: row.get("title"),
		author: row.get("author"),
		isb: row.get("isbn")
	})};*/
	Ok(books)
}
