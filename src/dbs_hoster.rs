use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(std::path::Path::new("dbs/").join(path))?)
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/files/{filename}", web::get().to(index)))
        .bind(("127.0.0.1", 9000))?
        .run()
        .await
}
