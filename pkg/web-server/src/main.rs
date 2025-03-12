use std::{fs::File, io::BufReader};

use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use rustls::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_config = load_server_config();

    HttpServer::new(|| App::new().service(ping))
        .bind_rustls_0_23(("127.0.0.1", 8080), server_config)?
        .run()
        .await
}

fn load_server_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder().with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open("./key/server.pem").unwrap());
    let cert_chain = CertificateDer::pem_reader_iter(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let key = PrivateKeyDer::from_pem_file("./key/server.key.pem").unwrap();

    config.with_single_cert(cert_chain, key).unwrap()
}
