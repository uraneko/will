use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpListener;
use std::path::PathBuf;

use super::files::rest::dir_component;

mod frontend;
mod logs;
mod request;
mod response;

pub(crate) use frontend::load_cache;
use request::{parse_body, parse_request, Request, RequestErr};
use response::{process_request_failure, process_request_success};

enum ServerErr {
    MalformedRequest,
    InsufficientHeaders,
}

pub(super) fn garcon(
    conn: TcpListener,
    src_files: &HashMap<&'static str, &'static str>,
    app_icons: &HashMap<&str, &str>,
    file_icons: &mut HashMap<String, String>,
    dirs: &mut HashMap<&PathBuf, String>,
    status: &HashMap<&str, &str>,
) {
    let mut data = String::new();
    let mut buf = vec![];

    while let Some(Ok(mut stream)) = conn.incoming().next() {
        let mut reader = std::io::BufReader::new(&mut stream);

        let mut request = String::new();
        loop {
            _ = reader.read_line(&mut request);

            if request.len() > "GET / HTTP/1.1".len() && request.ends_with("\r\n\r\n") {
                break;
            }
        }

        eprintln!("\r\n----------------------\r\n{:?}", request);

        let request = parse_request(&request);

        if let Err(e) = request {
            let response = match process_request_failure(e) {
                Ok(r) => r,
                Err(e) => {
                    println!("response error, aborting request reply, {:?}", e);
                    continue;
                }
            };

            let response = response.parse();
            let mut writer = std::io::BufWriter::new(&mut stream);
            _ = writer.write(response.as_bytes());
            continue;
        }

        let request = request.unwrap();

        // NOTE: im not sure whether to have
        // request.body take a mut ref and modify in place
        // or take self ans return self
        let request = parse_body(request, &mut data, &mut buf, &mut reader);

        let mut writer = std::io::BufWriter::new(&mut stream);

        println!("=========\r\n{:?}\r\n=========", request);

        if let (true, level) = request.is_bad() {
            let response = match process_request_failure(request.how_bad(level)) {
                Ok(r) => r,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
            let response = response.parse();
            _ = writer.write(response.as_bytes());
            continue;
        }
        // initialize response instance
        let mut clen = String::new();
        let response = match process_request_success(
            &request, src_files, dirs, file_icons, app_icons, status, &mut clen,
        ) {
            Ok(r) => r,
            Err(e) => {
                println!("response error, aborting request reply, {:?}", e);
                continue;
            }
        };

        println!("======{:?}======", &response);

        let response = response.parse();
        println!("======{:?}======", &response);
        _ = writer.write(response.as_bytes());

        // write response line, headers and body
    }
}
