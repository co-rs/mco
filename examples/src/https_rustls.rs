extern crate bytes;
extern crate httparse;
#[macro_use]
extern crate cogo;

use std::convert::TryInto;
use bytes::BufMut;
use httparse::Status;
use cogo::net::TcpListener;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::sync::Arc;
use rustls::{Certificate, OwnedTrustAnchor, PrivateKey, RootCertStore};
use rustls::server::Acceptor;

fn req_done(buf: &[u8], path: &mut String) -> Option<usize> {
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    if let Ok(Status::Complete(i)) = req.parse(buf) {
        path.clear();
        path.push_str(req.path.unwrap_or("/"));
        return Some(i);
    }
    None
}

fn main() {
    let f_cert = File::open("examples/rustls/cert.pem").unwrap();

    let mut reader = BufReader::new(f_cert);
    let cert= rustls_pemfile::certs(&mut reader).unwrap();
    let cert = cert[0].clone();

    let f_cert = File::open("examples/rustls/key.rsa").unwrap();
    let mut reader = BufReader::new(f_cert);
    let pris=rustls_pemfile::pkcs8_private_keys(&mut reader).unwrap();
    let pri = pris[0].clone();

    let private_key = PrivateKey(pri);//private.pem
    let cert = Certificate(cert);//cert

    let config = rustls::ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![cert], private_key).unwrap();
    let cfg = Arc::new(config);

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("server bind on https://127.0.0.1:8080");
    while let Ok((mut stream, _)) = listener.accept() {
        let mut conn = rustls::ServerConnection::new(cfg.clone()).unwrap();
        let mut stream = rustls::StreamOwned::new(conn, stream);
        go!(2*4096,move || {
            let mut buf = Vec::new();
            let mut path = String::new();

            loop {
                if let Some(i) = req_done(&buf, &mut path) {
                    let response = match &*path {
                        "/" => "Welcome to Cogo http demo\n",
                        "/hello" => "Hello, World!\n",
                        "/quit" => std::process::exit(1),
                        _ => "Cannot find page\n",
                    };

                    let s = format!(
                        "\
                         HTTP/1.1 200 OK\r\n\
                         Server: Cogo\r\n\
                         Content-Length: {}\r\n\
                         date: 1-1-2000\r\n\
                         \r\n\
                         {}",
                        response.len(),
                        response
                    );

                    stream
                        .write_all(s.as_bytes())
                        .expect("Cannot write to socket");

                    buf = buf.split_off(i);
                } else {
                    let mut temp_buf = vec![0; 512];
                    match stream.read(&mut temp_buf) {
                        Ok(0) => return, // connection was closed
                        Ok(n) => buf.put(&temp_buf[0..n]),
                        Err(err) => {
                            println!("err = {:?}", err);
                            break;
                        },
                    }
                }
            }
        });
    }
}
