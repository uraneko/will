use std::collections::LinkedList;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use crate::endpoints::{css, dir_template, html, js, ls, svg};
use crate::init::ServerMeta;

enum ResourceType {
    Icon,
    Dir,
    CSS,
    JS,
    HTML,
    Other(String),
}

enum HTMLMethod {
    GET,
    POST,
    Error,
}

struct HTMLResponse {
    path: PathBuf,
    method: HTMLMethod,
}

impl HTMLResponse {
    // makes a response from the received request headers
    // NOTE: you can only access resources/root
    // NOTE: css/js/html can all be hardcoded to dist/filename
    fn maker(params: LinkedList<String>, method: &str, root: &str) -> Self {
        // FIXME: if css or js append to endpoint path

        let method = match &method.to_uppercase()[..] {
            "GET" => HTMLMethod::GET,
            "POST" => HTMLMethod::POST,
            _ => HTMLMethod::Error,
        };

        let mut path = PathBuf::new();

        if params.is_empty() {
            path.push(root);
            path.push("index.html");
            return HTMLResponse { path, method };
        }

        Self {
            path: params.into_iter().fold(PathBuf::new(), |mut acc, i| {
                acc.push(match i.as_str() {
                    "i" => "resources/icons",
                    "d" => root,
                    "root" => "",
                    "index.html" => "dist/index.html",
                    "styles.css" => "dist/styles.css",
                    "main.js" => "dist/main.js",
                    other => &other,
                });
                acc
            }),
            method,
        }
    }

    fn content_type<'a>(&self) -> &'a str {
        // TODO: this is just a hack, should read and handle all request lines including
        // content-type
        match self.path.is_file() {
            true => match self.path.extension().unwrap().to_str().unwrap() {
                "svg" => "text/xml+svg",
                "html" => "text/html",
                "css" => "text/css",
                "js" => "application/js",
                _ => "unexpected content_type",
            },
            false => "text/html", // requested resource was a dir
        }
    }

    fn header(&self) -> Vec<u8> {
        let mut header = String::from("http/1.1 ");
        match self.content_type() {
            "text/html" => header += "200 OK",
            _ => header += "201 OK",
        };
        header.push_str("\r\nAccept-Range:bytes\r\nContent-Type:");
        header.push_str(self.content_type());
        header.push_str("; charset=utf-8\r\n\r\n");

        header.into_bytes()
    }

    fn body(&self) -> Vec<u8> {
        match self.path.is_dir() {
            true => self.resolve_dir(),
            false => match self.content_type() {
                "text/xml+svg" => self.resolve_icon(),
                "text/css" => self.resolve_css(),
                "application/js" => self.resolve_js(),
                "text/html" => self.resolve_html(),
                _ => self.reject_request(),
            },
        }
        .into_bytes()
    }

    fn reject_request(&self) -> String {
        "your request is rejected, tehe".into()
    }

    fn resolve_dir(&self) -> String {
        dir_template(ls(&self.path))
    }

    fn resolve_icon(&self) -> String {
        svg(&self.path.join(".svg"))
    }

    fn resolve_js(&self) -> String {
        match js(&self.path) {
            Ok(script) => script,
            Err(e) => format!(
                "failed to read requested js file contents {:?}\n{:?}",
                &self.path.file_name(),
                e
            ),
        }
    }

    fn resolve_css(&self) -> String {
        match css(&self.path) {
            Ok(styles) => styles,
            Err(e) => format!(
                "failed to read requested css file contents {:?}\n{:?}",
                &self.path.file_name(),
                e
            ),
        }
    }

    fn resolve_html(&self) -> String {
        match html(&self.path) {
            Ok(document) => document,
            Err(e) => format!(
                "failed to read requested html document contents {:?}\n{:?}",
                &self.path.file_name(),
                e
            ),
        }
    }
}

pub(crate) fn server_loop(listener: TcpListener, root: &str) {
    for mut stream in listener.incoming().flatten() {
        let mut reader = std::io::BufReader::new(&mut stream);
        eprintln!("{:?}", reader);
        // get resource
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let (header, body) = match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
            // TODO: security for received resource var
            // make sure received resource is not malicious string
            ["GET", uri, "HTTP/1.1"] => handle_get(&mut reader, uri, root),
            ["POST", uri, "HTTP/1.1"] => handle_post(&mut reader, uri, root),
            _ => todo!(),
        };

        stream.write_all(&header).unwrap();
        stream.write_all(&body).unwrap()
    }
}

fn parse_uri<'a>(uri: &str) -> LinkedList<String> {
    let mut chunks = uri.split("/").filter(|c| *c != "");

    let mut params = LinkedList::new();
    while let Some(chunk) = chunks.next() {
        params.push_back(chunk.to_string());
    }

    params
}

fn handle_get(reader: &mut BufReader<&mut TcpStream>, uri: &str, root: &str) -> (Vec<u8>, Vec<u8>) {
    let params = parse_uri(uri);
    loop {
        // TODO: handle request headers
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        print!("{}", &line[..]);

        if line.trim().is_empty() {
            break;
        }
    }
    println!("\r\n\r\n\r\n");

    // /i/home <- this is uri for getting home.svg icon from resources/icons
    // pop_back

    let res = HTMLResponse::maker(params, "get", root);

    (res.header(), res.body())
}

fn handle_post(
    reader: &mut BufReader<&mut TcpStream>,
    uri: &str,
    root: &str,
) -> (Vec<u8>, Vec<u8>) {
    todo!()
}
