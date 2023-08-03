use std::env;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use warp::Filter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Must specify mode.");
        return;
    }
    if args[1] == "server" {
        let server = run_server();
        server.await;
    } else {
        let client = run_client();
        client.await.unwrap();
    }
}

async fn run_server() {
    let routes = warp::any().map(|| "Hello from mTLS.");

    warp::serve(routes)
        .tls()
        .key_path("ca/server.key")
        .cert_path("ca/server.crt")
        .client_auth_required_path("ca/server.bundle.crt")
        .run(([0, 0, 0, 0], 3030))
        .await
}

async fn run_client() -> Result<(), reqwest::Error> {
    let ca_cert_file = read_file("ca/ca.crt").await;
    let cert = reqwest::Certificate::from_pem(&ca_cert_file)?;
    
    let client_pem_file = read_file("ca/client.pem").await;
    let identity = reqwest::Identity::from_pem(&client_pem_file)
        .unwrap();

    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .tls_built_in_root_certs(false)
        .add_root_certificate(cert)
        .identity(identity)
        .https_only(true)
        .build()?;

    let res = client.get("https://localhost:3030")
        .send()
        .await
        .unwrap();

    println!("Received: {:?}", res.text().await.unwrap());

    Ok(())
}

async fn read_file(file_name: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    File::open(file_name)
        .await
        .unwrap()
        .read_to_end(&mut buf)
        .await
        .unwrap();
    return buf;
}
