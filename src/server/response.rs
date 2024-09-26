use std::collections::HashMap;
use std::path::PathBuf;

use super::frontend::src_files;
use super::{Request, RequestErr};

#[derive(Debug)]
pub(crate) struct ResponseLine<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
}

#[derive(Debug)]
pub(crate) struct Response<'a> {
    response_line: ResponseLine<'a>,
    headers: HashMap<&'a str, &'a str>,
    body: HashMap<&'a str, &'a str>,
}

pub(crate) fn process_request_success<'a>(
    req: &'a Request<'a>,
    src_files: &'a HashMap<&str, &str>,
    dirs: &'a HashMap<&PathBuf, String>,
    file_icons: &'a HashMap<String, String>,
    app_icons: &'a HashMap<&str, &str>,
    status: &'a HashMap<&str, &str>,
    clen: &'a mut String,
) -> Result<Response<'a>, ResponseErr> {
    if req.method() == "GET" && req.no_params() {
        println!("**{}", req.resource());
        match src_files.contains_key(req.resource()) {
            true => {
                let file = src_files.get(req.resource()).unwrap();
                *clen = format!("{}", file.len());
                let cty = req.mime_type().unwrap_or("text/html");

                return Ok(response_template(status.status("200"), req.version())
                    .headers(HashMap::from([
                        ("Content-Type", cty),
                        ("Content-Length", clen.as_str()),
                    ]))
                    .body(HashMap::from([("data", *file)])));
            }
            false => {
                return Ok(response_template(status.status("204"), req.version())
                    .headers(HashMap::from([("Content-Lnegth", "0")])))
            }
        }
    }

    Err(ResponseErr::MethodUnsupported)
}

// TODO: should have the request error variant instance
// hold all the necessary data from the request
pub(crate) fn process_request_failure<'a>(
    req_err: RequestErr,
) -> Result<Response<'a>, ResponseErr> {
    match req_err {
        _ => todo!(),
    }
}

// pub(crate) fn process_request<'a>(
//     maybe_request: Result<Request, RequestErr>,
//
//     src_files: &HashMap<&str, &str>,
//     dirs: &HashMap<&PathBuf, String>,
//     file_icons: &HashMap<String, String>,
//     app_icons: &HashMap<&str, &str>,
//     status: &HashMap<&str, &str>,
// ) -> Result<Response<'a>, ResponseErr> {
//     if let Ok(request) = maybe_request {
//         process_request_success(&request, src_files, dirs, file_icons, app_icons, status)
//     } else if let Err(req_err) = maybe_request {
//         process_request_failure(req_err)
//     } else {
//         unreachable!("a result can only be Ok or Err")
//     }
// }

// generates the response headers
fn generate_headers<'a>(method: &'a str, clen: &'a str) -> HashMap<&'a str, &'a str> {
    let mut headers = HashMap::new();
    if method == "GET" {
        headers.insert("Content-Length", clen)
    } else {
        todo!()
    };

    todo!()
}

// generates the response body
fn generate_body<'a>() -> HashMap<&'a str, &'a str> {
    todo!()
}

fn process_request_line(
    req: &Request,
    headers: &mut HashMap<&str, &str>,
    body: &mut HashMap<&str, &str>,
) {
}

// specifies the requested resource type
// source file, html component,
fn resource_type() -> u8 {
    todo!()
}

#[derive(Debug)]
pub(crate) enum StatusErr {
    FileNotFound,
}

// do the functions that generate response stuff
fn status_code<'a>(status: &'a Result<Option<&str>, StatusErr>) -> &'a str {
    if let Err(e) = status {
        return match e {
            StatusErr::FileNotFound => "",
        };
    }

    let status = status.as_ref().unwrap();

    match status {
        Some(data) => "",
        None => "",
    }
}

fn status_text(status_code: &str) -> Result<&str, ResponseErr> {
    match status_code {
        "100" => Ok("Continue"),
        "101" => Ok("Switching Protocols"),
        "200" => Ok("OK"),
        "201" => Ok("Created"),
        "202" => Ok("Accepted"),
        "204" => Ok("No Content"),
        "303" => Ok("See Other"),
        "304" => Ok("Not Modified"),
        "307" => Ok("Temporary Redirect"),
        "308" => Ok("Permanent Redirect"),
        "400" => Ok("Bad Request"),
        "401" => Ok("Unauthorized"),
        "403" => Ok("Forbidden"),
        "404" => Ok("Not Found"),
        "405" => Ok("Method Not Allowed"),
        "406" => Ok("Not Acceptable"),
        "408" => Ok("Request Timeout"),
        "410" => Ok("Gone"),
        "500" => Ok("Internal Server Error"),
        "501" => Ok("Not Implemented"),
        "502" => Ok("Bad Gateway"),
        "503" => Ok("Service Unavailable"),
        "504" => Ok("Gateway Timeout"),
        _ => Err(ResponseErr::UnsupportedStatusCode),
    }
}

// NOTE: GET, POST  need a response body
// NOTE: PUT,   dont need a response body

fn write_response(response: Response, writer: &mut std::io::BufWriter<&mut std::net::TcpStream>) {}

#[derive(Debug)]
pub(crate) enum ResponseErr {
    UnrecognizedStatusCode,
    UnsupportedStatusCode,
    MethodUnsupported,
}

impl<'a> Response<'a> {
    fn new() -> Self {
        Self {
            response_line: ResponseLine::<'a> {
                status_code: "",
                version: "",
                status_text: "",
            },
            body: HashMap::new(),
            headers: HashMap::new(),
        }
    }

    fn status_code(self, status_code: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_code,
                ..self.response_line
            },
            ..self
        }
    }

    fn status_text(self, status_text: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                status_text,
                ..self.response_line
            },
            ..self
        }
    }

    fn version(self, version: &'a str) -> Self {
        Self {
            response_line: ResponseLine {
                version,
                ..self.response_line
            },
            ..self
        }
    }

    fn body(self, body: HashMap<&'a str, &'a str>) -> Self {
        Self {
            response_line: self.response_line,
            body,
            ..self
        }
    }

    fn headers(self, headers: HashMap<&'a str, &'a str>) -> Self {
        Self {
            response_line: self.response_line,
            headers,
            ..self
        }
    }

    fn body_mut(&mut self, body: HashMap<&'a str, &'a str>) {
        self.body = body;
    }

    pub fn parse(self) -> String {
        format!(
            "{} {} {}\r\n{}\r\n{}}}",
            self.response_line.version,
            self.response_line.status_code,
            self.response_line.status_text,
            self.headers
                .into_iter()
                .fold(String::new(), |acc, (k, v)| acc + k + ": " + v + "\r\n"),
            self.body
                .into_iter()
                .fold(String::from("{"), |acc, (k, v)| {
                    acc + "{" + k + ": " + v + ","
                })
        )
    }

    fn is_ready(&self) -> bool {
        if self.response_line.version.is_empty()
            || self.response_line.status_text.is_empty()
            || self.response_line.status_code.is_empty()
            || self.headers.is_empty()
            || self.body.is_empty()
        {
            return true;
        }

        false
    }
}

const STATUS: [(&str, &str); 23] = [
    ("100", "Continue"),
    ("101", "Switching Protocols"),
    ("200", "OK"),
    ("201", "Created"),
    ("202", "Accepted"),
    ("204", "No Content"),
    ("303", "See Other"),
    ("304", "Not Modified"),
    ("307", "Temporary Redirect"),
    ("308", "Permanent Redirect"),
    ("400", "Bad Request"),
    ("401", "Unauthorized"),
    ("403", "Forbidden"),
    ("404", "Not Found"),
    ("405", "Method Not Allowed"),
    ("406", "Not Acceptable"),
    ("408", "Request Timeout"),
    ("410", "Gone"),
    ("500", "Internal Server Error"),
    ("501", "Not Implemented"),
    ("502", "Bad Gateway"),
    ("503", "Service Unavailable"),
    ("504", "Gateway Timeout"),
];

pub(crate) fn status() -> HashMap<&'static str, &'static str> {
    HashMap::from(STATUS)
}

fn response_template<'a>(status: (&'a str, &'a str), version: &'a str) -> Response<'a> {
    Response {
        response_line: ResponseLine {
            version,
            status_code: status.0,
            status_text: status.1,
        },
        headers: HashMap::new(),
        body: HashMap::new(),
    }
}

trait Status<'a> {
    fn status(&'a self, code: &'a str) -> (&str, &str);
}

impl<'a> Status<'a> for HashMap<&'a str, &'a str> {
    fn status(&'a self, code: &'a str) -> (&str, &str) {
        (code, self.get(code).unwrap())
    }
}
