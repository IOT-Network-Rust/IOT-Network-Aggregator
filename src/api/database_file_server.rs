use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file_path = std::path::Path::new("dbs/").join(path);
    Ok(NamedFile::open(file_path)?)
}

#[actix_web::main]
pub async fn start_server(ip_addr:&str, port:u16) -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/files/{filename}", web::get().to(index)))
        .bind((ip_addr, port))?
        .run()
        .await
}
