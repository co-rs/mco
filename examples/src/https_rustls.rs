extern crate bytes;
extern crate httparse;
#[macro_use]
extern crate mco;

use bytes::BufMut;
use httparse::Status;
use mco::net::TcpListener;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Write};
use std::sync::Arc;

// This example is for demonstration only and is suitable for production environment please move on
// example see https://github.com/co-rs/mco-http/tree/main/examples
fn main() {
    let config = {
        let mut f_cert = File::open("examples/rustls/sample.pem").unwrap();
        let mut f_key = File::open("examples/rustls/sample.rsa").unwrap();

        let mut certs =vec![];
        _ = f_cert.read_to_end(&mut certs);

        let mut key =vec![];
        _ = f_key.read_to_end(&mut key);

        let flattened_data: Vec<u8> = vec![certs].into_iter().flatten().collect();
        let mut reader = BufReader::new(Cursor::new(flattened_data));
        let certs = rustls_pemfile::certs(&mut reader).map(|result| result.unwrap())
            .collect();
        let private_key=rustls_pemfile::private_key(&mut BufReader::new(Cursor::new(key.clone()))).expect("rustls_pemfile::private_key() read fail");
        if private_key.is_none() {
            panic!("load keys is empty")
        }
        rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key.unwrap()).unwrap()
    };

    let cfg = Arc::new(config);

    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("server bind on https://127.0.0.1:3000");
    while let Ok((mut stream, _)) = listener.accept() {
        let mut conn = rustls::ServerConnection::new(cfg.clone()).unwrap();
        let mut stream = rustls::StreamOwned::new(conn, stream);
        co!(2 * 4096, move || {
            let mut buf = Vec::new();
            let mut path = String::new();

            loop {
                if let Some(i) = req_done(&buf, &mut path) {
                    let response = match &*path {
                        "/" => "Welcome to mco http demo\n",
                        "/hello" => "Hello, World!\n",
                        "/quit" => std::process::exit(1),
                        _ => "Cannot find page\n",
                    };

                    let s = format!(
                        "\
                         HTTP/1.1 200 OK\r\n\
                         Server: mco\r\n\
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
                        }
                    }
                }
            }
        });
    }
}

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
