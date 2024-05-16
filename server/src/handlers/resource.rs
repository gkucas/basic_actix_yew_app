use std::io::Error;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, HttpRequest};

#[get("/assets/{filename:.*}")]
async fn assets(req: HttpRequest) -> Result<NamedFile, Error> {
    let file_name: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path= String::from("app/dist/") + file_name.file_name().unwrap().to_str().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/{tail:.*}")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "app/dist/index.html".parse().unwrap();
    return Ok(NamedFile::open(path).expect("File not found"))
}