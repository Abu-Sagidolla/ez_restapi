
use crate::models::Report;
use std::error::Error;
use sqlx::Row;
use sqlx::FromRow;
use sqlx::query;
use serde_json::json;
use chrono::DateTime;
use chrono::Utc;
use chrono::TimeZone;
use sqlx::types::Json;

pub async fn create(report:&Report,pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
     let query = "INSERT INTO Scanreport (scandata,scanned) VALUES ($1,$2) ";
     let scanned_utc: Option<DateTime<Utc>> = report.scanned.map(|dt| Utc.from_utc_datetime(&dt));
     sqlx::query(query)
         .bind(Json(serde_json::to_value(&report.scan_data).unwrap()))
         .bind(scanned_utc)
         .execute(pool).await?;

    Ok(())
}

//just little bit complicated way to create book
/*async fn  insert_transaction(
	book: Report, conn: &sqlx::PgPool
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
} */


pub async fn update(report:&Report,pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
     let query = "UPDATE INTO Scanreport (scandata,scanned) VALUES ($1,$2) ";

     let scanned_utc: Option<DateTime<Utc>> = report.scanned.map(|dt| Utc.from_utc_datetime(&dt));
     sqlx::query(query)
         .bind(serde_json::to_value(&report.scan_data).unwrap())
         .bind(scanned_utc)
         .execute(pool).await?;

    Ok(())
}

pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Report>, Box<dyn Error>>{
	let q = "SELECT * FROM Scanreport";
	let query = sqlx::query(q);

    let reports = query.fetch_all(conn).await?;
	/*while let Some(row) = rows.try_next().await? {
		books.push(Book {
		title: row.get("title"),
		author: row.get("author"),
		isb: row.get("isbn")
	})};*/
	Ok(reports)
}
