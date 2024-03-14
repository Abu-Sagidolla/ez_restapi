use std::error::Error;
use sqlx::Connection;
use sqlx::Row;
use dotenv::dotenv;
use std::env;

mod crud;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> 
{
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("MAILCOACH_API_TOKEN must be set.");
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    
    //migration 
    //sqlx::migrate!("./migrations").run(&pool).await?;
    
    let new_book = models::Book {
        title: "Anti qotaqbastar".to_string(),
        author: "Qutluq".to_string(),
        isbn: "99-112-512-123".to_string()
    };
    //crud::create(&new_book,&pool).await?;
    println!("{:?}",crud::read(&pool).await?);
    Ok(())
}
