
use chrono::prelude::*;
use serde::{ Deserialize, Serialize };
use db::DB;
use std::convert::Infallible;
use warp::{ Filter, Rejection };

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handlers;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub num_pages: usize,
    pub added_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + clone {
    warp::any().map(move || db.clone())
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;
    let book = warp::path("book");

    let book_routes = book
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::create_book_handler)
        .or(book
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::edit_book_handler))
        .or(book
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_book_handler))
        .or(book
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::books_list_handler));


    let routes = book_routes.recover(error::handle_rejection);
    println!("Started on port 8080");

    warp::serve(routes)
            .run(([127, 0, 0, 1], 8080))
            .await;

    Ok(())
}
