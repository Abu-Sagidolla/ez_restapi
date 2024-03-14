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
use chrono::NaiveDateTime;
use crate::models::Data;

pub async fn create(report:&Report,pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
     let query = "INSERT INTO Scanreport (id,scandata,scanned) VALUES ($1,$2,$3) ";

     sqlx::query(query)
         .bind(&report.id)
         .bind(&report.scan_data)
         .bind(report.scanned)
         .execute(pool).await?;

    Ok(())
}


pub async fn update(report:&Report,pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
     let query = "UPDATE INTO Scanreport (scandata,scanned) VALUES ($1,$2,$3) ";

     sqlx::query(query)
         .bind(&report.id)
         .bind(&report.scan_data)
         .bind(report.scanned)
         .execute(pool).await?;

    Ok(())
}

pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Report>, Box<dyn Error>> {
    let q = "SELECT * FROM Scanreport";
    let reports = sqlx::query(q).fetch_all(conn).await?;

    let mut output: Vec<Report> = Vec::new();
    for row in reports.iter() {
        let scan_data: Json<Data> = row.try_get("scandata")?;
        let scanned: NaiveDateTime = row.try_get("scanned")?;
        let id = row.try_get("id")?;
        //println!("{:?}", scan_data); // For debugging
        output.push(Report { scan_data, scanned,id });
    }

    Ok(output)
}
