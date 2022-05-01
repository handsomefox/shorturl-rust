mod database;
mod entry;
mod shortener;

use crate::database::Database;
use actix_files::NamedFile;
use actix_web::http::header::LOCATION;
use actix_web::{error, get, web, App, HttpRequest, HttpResponse, HttpServer};
use std::env;
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile, error::Error> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
#[get("api/s/{base64}")]
async fn short(db: web::Data<Database>, base64: web::Path<String>) -> HttpResponse {
    let res = db.get_shortened(base64.into_inner()).await;
    match res {
        Ok(entry) => HttpResponse::Ok().body(format!("{{\"link\": \"{}\"}}", entry.short)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("api/u/{hash}")]
async fn unroll(db: web::Data<Database>, hash: web::Path<String>) -> HttpResponse {
    let res = db.get_unrolled(hash.into_inner()).await;
    match res {
        Ok(entry) => HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, entry.url))
            .finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_uri = env::var("MONGO_URI").expect("You must set the MONGODB_URI environment var!");
    let port = env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .unwrap();

    let db_conn = Database::new(client_uri.as_str()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_conn.clone()))
            .route("/", web::get().to(index))
            .service(short)
            .service(unroll)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
