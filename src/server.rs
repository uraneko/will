use std::ffi::OsStr;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpListener;
use std::path::PathBuf;

mod request;
mod response;

use request::{parse_request, RequestErr};

enum ServerErr {
    MalformedRequest,
    InsufficientHeaders,
}

pub(super) fn garcon(conn: TcpListener) {
    while let Some(Ok(mut stream)) = conn.incoming().next() {
        let mut reader = std::io::BufReader::new(&mut stream);

        let mut request = String::new();
        loop {
            _ = reader.read_line(&mut request);

            if request.len() > 4 && request.ends_with("\r\n\r\n") {
                break;
            }
        }

        eprintln!("\r\n----------------------\r\n{:?}", request);

        let request = parse_request(&request);

        let mut writer = std::io::BufWriter::new(&mut stream);

        if let Err(RequestErr::BadRequestLine {
            method,
            version,
            url,
        }) = request
        {
            eprintln!(
                "bad {} request at url {} with http version {}, aborting...",
                method, url, version
            );
            _ = writer.write(format!("{} 400 Bad Request", version).as_bytes());
            continue;
        } else if let Err(e) = request {
            eprintln!("server aborting request due to error: {:?}", e);
            continue;
        }

        let request = request.unwrap();

        println!("=========\r\n{:?}\r\n=========", request);

        if request.is_bad() {
            let Some(bad) = request.how_bad() else {
                unreachable!("not bad after all")
            };
            println!("the request is bad, {}", bad);
            continue;
        }
        // initialize response instance

        // write response line, headers and body
    }
}
