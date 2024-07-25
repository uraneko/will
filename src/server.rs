use std::io::BufRead;
use std::net::TcpListener;

pub(super) const ROOT_DIR: &str = "resources/root/";

pub(super) fn server(conn: TcpListener) {
    for mut stream in conn.incoming().flatten() {
        let mut reader = std::io::BufReader::new(&mut stream);

        let mut request = String::new();
        while reader.has_data_left().unwrap() {
            _ = reader.read_line(&mut request);
        }

        eprintln!("\r\n{:?}", request);
        let mut req_lines = request.lines();

        let method = formulate_request(&req_lines.next().unwrap());
        eprintln!("\r\n{:?}", method);
    }
}

#[derive(Debug)]
enum Method<'a> {
    GET { uri: &'a str, protocol: &'a str },
    POST { uri: &'a str, protocol: &'a str },
    None,
    // ...
}

impl<'a> Method<'a> {
    // TODO: if protocol is not some http version return error
    fn new(method: &str, mut chunks: std::str::Split<'a, char>) -> Self {
        match method {
            "GET" => Self::GET {
                uri: chunks.next().unwrap(),
                protocol: chunks.next().unwrap(),
            },
            "POST" => Self::POST {
                uri: chunks.next().unwrap(),
                protocol: chunks.next().unwrap(),
            },
            _ => {
                eprintln!("this method is on vacation");
                Self::None
            }
        }
    }
}

fn formulate_request(req_str: &str) -> Method {
    let mut res_chunks = req_str.split(' ');
    match res_chunks.next() {
        Some(method) => Method::new(method, res_chunks),
        None => Method::None,
    }
}
