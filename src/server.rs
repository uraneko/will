use std::ffi::OsStr;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;

use std::net::TcpListener;

pub(super) const ROOT_DIR: &str = "resources/root/";

pub(super) fn server(conn: TcpListener) {
    while let Some(Ok(mut stream)) = conn.incoming().next() {
        let mut reader = std::io::BufReader::new(&mut stream);

        let mut request = String::new();
        while request.len() < 4 || &request[request.len() - 4..] != "\r\n\r\n" {
            _ = reader.read_line(&mut request);
        }

        eprintln!("\r\n{:?}", request);
        let mut req_lines = request.lines();

        let [method, uri, http_ver] = formulate_request(&req_lines.next().unwrap());

        let mut writer = std::io::BufWriter::new(&mut stream);
        if method.is_empty() || uri.is_empty() || http_ver.is_empty() {
            eprintln!(
                "received bad request\r\nmethod: {}\r\nuri: {}\r\nhttp_ver: {}\r\naborting request",
                method, uri, http_ver
            );
            _ = writer.write((http_ver.to_string() + " 400 Bad Request").as_bytes());
            continue;
        }

        let resource_path = ROOT_DIR.to_string() + uri;

        let response_body = fetch_resource(&resource_path);
        let mut response_headers =
            response_headers(&http_ver, &response_body, &PathBuf::from(&uri)).into_iter();

        while let Some(header) = response_headers.next() {
            eprintln!("{}", String::from_utf8_lossy(&header));
            _ = writer.write(&header);
            _ = writer.write(&[13, 10]);
        }
        _ = writer.write(&[13, 10]);

        _ = writer.write(&response_body.as_bytes());
        _ = writer.flush();
    }
}

// TODO: if protocol is not some http version return error
fn formulate_request(req_str: &str) -> [&str; 3] {
    let mut res_chunks = req_str.split(' ');

    // assert_eq!(res_chunks.len(), 3);
    res_chunks.next_chunk::<3>().unwrap_or(["", "", ""])
}

fn fetch_resource(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{:?}", e);
            "".into()
        }
    }
}
// TODO: impl http's own error system 4xx instead of my own
fn response_headers(http_ver: &str, data: &str, path: &PathBuf) -> Vec<Vec<u8>> {
    let mut headers: Vec<Vec<u8>> = vec![];

    let status_line = match data.is_empty() {
        true => http_ver.to_string() + " 404 NotFound",
        false => http_ver.to_string() + " 200 OK",
    };

    // let accept_range = "Accept-Range: bytes".to_string();

    let content_type = "Content-Type: ".to_string() + &content_type(path);

    let content_length = "Content-Length: ".to_string() + &data.len().to_string();

    headers.extend(
        [
            status_line.into_bytes(),
            // accept_range.into_bytes(),
            content_type.into_bytes(),
            content_length.into_bytes(),
        ]
        .into_iter(),
    );

    headers
}

fn content_type<'a>(path: &PathBuf) -> String {
    // FIXME:
    let ty = path
        .extension()
        .unwrap_or(OsStr::new(".html"))
        .to_str()
        .unwrap_or(".html");

    match ty {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "jpg" | ".jpeg" => "image/jpeg",
        "mp3" => "audio/mpeg",
        "svg" => "image/svg+xml",
        "tar" => "application/x-tar",
        "txt" => "text/plain",
        "ttf" => "font/ttf",
        "xhtml" => "application/xhtml+xml",
        _ => "",
    }
    .to_string()
    // + "; charset=utf-8"
}
