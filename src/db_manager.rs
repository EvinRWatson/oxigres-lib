use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::fs::File;
use std::io::Read;
use openssl::x509::X509;

fn load_root_certificate(cert_path: &str) -> X509 {
    let cert_file = File::open(cert_path);
    let mut cert_data = Vec::new();
    let _ = cert_file.unwrap().read_to_end(&mut cert_data);
    let certificate = X509::from_pem(&cert_data);
    return certificate.unwrap();
}

pub fn postgres_connect() -> postgres::Client {
    let pg_host = std::env::var("OXIGRES_DB_HOST").expect("missing environment variable OXIGRES_DB_HOST");
    let pg_port = std::env::var("OXIGRES_DB_PORT").expect("missing environment variable OXIGRES_DB_PORT");
    let pg_user = std::env::var("OXIGRES_DB_USER").expect("missing environment variable OXIGRES_DB_USER");
    let pg_password = std::env::var("OXIGRES_DB_PASSWD").expect("missing environment variable OXIGRES_DB_PASSWD");
    let pg_dbname = std::env::var("OXIGRES_DB_NAME").expect("missing environment variable OXIGRES_DB_NAME");
    let pg_cert_path = std::env::var("OXIGRES_DB_CERT_PATH").expect("missing environment variable OXIGRES_DB_CERT_PATH");

    let cert: X509 = load_root_certificate(&pg_cert_path);

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.cert_store_mut().add_cert(cert).unwrap();
    builder.set_verify(SslVerifyMode::PEER);

    let tls_connector = MakeTlsConnector::new(builder.build());

    let url = format!(
        "host={} port={} user={} password={} dbname={} sslmode=require",
        pg_host, pg_port, pg_user, pg_password, pg_dbname
    );
    let pg_client = postgres::Client::connect(&url, tls_connector).expect("failed to connect to postgres");

    return pg_client;
}