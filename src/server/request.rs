use std::collections::HashMap;
use std::io::Read;

const MIME_TYPES: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8";

pub(crate) fn parse_request(request: &str) -> Result<Request, RequestErr> {
    let mut lines = request.lines();

    let reqline = lines.next();
    if reqline.is_none() {
        return Err(RequestErr::EmptyRequestLine);
    }

    // parse the entire request line
    let req_line = parse_request_line(reqline.unwrap());

    let Ok([method, url, version]) = req_line else {
        return Err(RequestErr::BadRequest);
    };

    // check req method validity; is GET, POST or some other HTTP request method
    if !is_http_method(method) || !is_http_version(version) {
        return Err(RequestErr::BadRequestLine {
            method,
            url,
            version,
        });
    }

    // parse the reqline url domain and params

    let (url_path, url_params) = if let Ok((Ok(path), params)) = parse_url(url) {
        (path, params)
    } else {
        return Err(RequestErr::BadRequest);
    };

    let lines = lines.collect::<Vec<&str>>();
    let headers = parse_headers(lines);

    Ok(Request::new(
        method,
        version,
        url,
        url_path,
        url_params,
        headers,
        HashMap::new(),
    ))
}

fn is_supported_request(request: &str) -> Option<&str> {
    if is_head_request(request) {
        Some("HEAD")
    } else if is_get_request(request) {
        Some("GET")
    } else if is_post_request(request) {
        Some("POST")
    } else if is_put_request(request) {
        Some("PUT")
    } else if is_delete_request(request) {
        Some("DELETE")
    } else {
        None
    }
}

fn is_delete_request(request: &str) -> bool {
    request.starts_with("DELETE")
}

fn is_put_request(request: &str) -> bool {
    request.starts_with("PUT")
}

fn is_get_request(request: &str) -> bool {
    request.starts_with("GET")
}

fn is_post_request(request: &str) -> bool {
    request.starts_with("POST")
}

fn is_head_request(request: &str) -> bool {
    request.starts_with("HEAD")
}

fn is_http_method(method: &str) -> bool {
    match method {
        // NOTE: validating only get and post
        // because this server will no thandle anything else
        "HEAD" | "GET" | "POST" | "PUT" | "DELETE" => true,
        _ => false,
    }
}

fn is_http_version(version: &str) -> bool {
    match version {
        // NOTE: QUIC is too ambitious, remove it
        "HTTP/1.0" | "HTTP/1.1" | "HTTP/2" | "HTTP/3" => true,
        _ => false,
    }
}

fn parse_request_line(line: &str) -> Result<[&str; 3], RequestErr> {
    let [method, url, version] = {
        let mut s = line.split_whitespace();
        [s.next(), s.next(), s.next()]
    };

    if method.is_none() || url.is_none() || version.is_none() {
        return Err(RequestErr::BadRequestLine {
            method: method.unwrap_or("got no method"),
            url: url.unwrap_or("got no url"),
            version: version.unwrap_or("got no version"),
        });
    }

    Ok([method.unwrap(), url.unwrap(), version.unwrap()])
}

const WEB_RESOURCES: &str = "resources/frontend/";

fn parse_url(url: &str) -> Result<(Result<PathBuf, RequestErr>, HashMap<&str, &str>), RequestErr> {
    let [target, params] = {
        let mut s = url.splitn(2, "???");
        [s.next(), s.next()]
    };

    let target = if let Some(target) = target {
        parse_url_target(target)
    } else {
        return Err(RequestErr::URLTargetNotFound);
    };

    let params = if let Some(params) = params {
        parse_url_params(params)
    } else {
        HashMap::new()
    };

    Ok((target, params))
}

use std::path::PathBuf;

// TODO: when request returns an error we dont just abort
// instead server should return a suitable 4xx/5xx response

fn parse_url_target(target: &str) -> Result<PathBuf, RequestErr> {
    let target = WEB_RESOURCES.to_string() + if target == "/" { "/index.html" } else { target };
    let path = PathBuf::from(&target);

    println!("[[[[[[\n{:?}\n]]]]]]", path);
    match path.is_file() {
        true => Ok(path),
        false => Err(RequestErr::TargetUnparsable),
    }
}

fn parse_url_params(params: &str) -> HashMap<&str, &str> {
    params
        .split('&')
        .map(|p| parse_url_param(p))
        .filter(|res| res.is_ok())
        .map(|qp| qp.unwrap())
        .collect()
}

fn parse_url_param(param: &str) -> Result<(&str, &str), RequestErr> {
    let mut s = param.splitn(2, "=");
    // println!("k: {:?}, v: {:?}", s.next(), s.next());
    Ok((
        match s.next() {
            Some(p) => p,
            None => return Err(RequestErr::ParamNameNotFound),
        },
        match s.next() {
            Some(v) => v,
            None => return Err(RequestErr::ParamValueNotFound),
        },
    ))
}

fn parse_headers(lines: Vec<&str>) -> HashMap<&str, &str> {
    lines
        .into_iter()
        .map(|l| parse_header(l))
        // .inspect(|r| println!("parsed header: {:?}", r))
        .filter(|h| h.is_ok())
        .map(|h| h.unwrap())
        .collect()
}

fn parse_header(line: &str) -> Result<(&str, &str), RequestErr> {
    let mut s = line.splitn(2, ": ");
    // println!("k: {:?}, v: {:?}", s.next(), s.next());
    Ok((
        match s.next() {
            Some(k) => k,
            None => return Err(RequestErr::HeaderKeyNotFound),
        },
        match s.next() {
            Some(v) => v,
            None => return Err(RequestErr::HeaderValueNotFound),
        },
    ))
}

fn parse_fields<'a>(lines: Vec<&'a str>) -> HashMap<&'a str, &'a str> {
    lines
        .into_iter()
        .filter(|l| !["{", "[", "}", "]"].contains(&l.trim()))
        .map(|l| parse_field(l))
        // .inspect(|r| println!("parsed request body field: {:?}", r))
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap())
        .collect()
}

fn parse_field(line: &str) -> Result<(&str, &str), RequestErr> {
    let mut s = line.splitn(2, ": ");
    // println!("k: {:?}, v: {:?}", s.next(), s.next());
    Ok((
        match s.next() {
            Some(k) => k,
            None => return Err(RequestErr::HeaderKeyNotFound),
        },
        match s.next() {
            Some(v) => v,
            None => return Err(RequestErr::HeaderValueNotFound),
        },
    ))
}

pub(crate) fn parse_body<'a>(
    request: Request<'a>,
    data: &'a mut String,
    buf: &'a mut Vec<u8>,
    reader: &mut std::io::BufReader<&mut std::net::TcpStream>,
) -> Request<'a> {
    if !request.has_body() {
        return request;
    }

    let len = request.body_len();

    buf.clear();
    buf.resize(len, 0);
    _ = reader.read_exact(buf);

    *data = String::from_utf8(buf.to_vec()).unwrap();

    let lines = data.lines().collect::<Vec<&str>>();

    let body = parse_fields(lines);
    request.body(body)
}

// enum Request {
//     Get {
//         version: f32,
//         uri: &'static str,
//         params: HashMap<&'static str, &'static str>,
//         headers: HashMap<&'static str, &'static str>,
//     },
//
//     Post {
//         version: f32,
//         url: &'static str,
//         body: &'static str,
//         headers: HashMap<&'static str, &'static str>,
//     },
//
//     Bad(RequestErr),
// }

#[derive(Debug)]
pub(crate) enum RequestErr<'a> {
    UnsupportedFormat,
    URLTargetNotFound,
    ParamNameNotFound,
    ParamValueNotFound,
    TargetUnparsable,
    BadHeaders,
    BadRequestLine {
        method: &'a str,
        url: &'a str,
        version: &'a str,
    },
    BadRequest,
    EmptyRequestLine,
    UnrecognizedMethod,
    HeaderKeyNotFound,
    HeaderValueNotFound,
}
// requestline - headers - optional body

#[derive(Debug)]
pub(crate) struct RequestLine<'a> {
    version: &'a str,
    method: &'a str,
    url: &'a str,
    url_path: PathBuf,
    url_params: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
pub(crate) struct Request<'a> {
    request_line: RequestLine<'a>,
    headers: HashMap<&'a str, &'a str>,
    body: HashMap<&'a str, &'a str>,
}

impl<'a> Request<'a> {
    fn new(
        method: &'a str,
        version: &'a str,
        url: &'a str,
        url_path: PathBuf,
        url_params: HashMap<&'a str, &'a str>,
        headers: HashMap<&'a str, &'a str>,
        body: HashMap<&'a str, &'a str>,
    ) -> Self {
        Self {
            request_line: RequestLine {
                method,
                version,
                url,
                url_path,
                url_params,
            },
            headers,
            body,
        }
    }

    pub(super) fn body(mut self, body: HashMap<&'a str, &'a str>) -> Self {
        self.body = body;
        self
    }

    pub(super) fn method(&self) -> &str {
        self.request_line.method
    }

    pub(super) fn version(&self) -> &str {
        self.request_line.version
    }

    pub(super) fn url(&self) -> &str {
        self.request_line.url
    }

    pub(super) fn is_http2(&self) -> bool {
        self.request_line.version == "HTTP/2"
    }

    pub(super) fn is_http1_1(&self) -> bool {
        self.request_line.version == "HTTP/1.1"
    }

    pub(super) fn is_http3(&self) -> bool {
        self.request_line.version == "HTTP/3"
    }

    pub(super) fn is_http1_0(&self) -> bool {
        self.request_line.version == "HTTP/1.0"
    }

    // this is needed because e.g.,
    // we could get a post request without body or content type
    pub(crate) fn is_bad(&self) -> (bool, u8) {
        if self.is_http2()
            && ["Transfer-Encoding", "Upgrade", "Connection"]
                .into_iter()
                .any(|k| self.headers.contains_key(k))
        {
            return (true, 1);
        } else if self.is_http1_1() && ["Host"].into_iter().any(|k| !self.headers.contains_key(k)) {
            return (true, 2);
        } else if self.transfer_encoding().is_some()
            || self.content_length().is_some() && self.body.is_empty()
        {
            return (true, 3);
        }

        (false, 0)
    }

    pub(crate) fn how_bad(&self, level: u8) -> RequestErr {
        match level {
            _ => todo!(),
        }
    }

    pub(crate) fn is_secure(&self) -> bool {
        false
    }

    fn has_body(&self) -> bool {
        self.headers.contains_key("Content-Length")
    }

    // TODO: match by request url and content length to parse body correctly
    // FIXME: validate len unwrap and from_utf8 unwrap
    // if error then return response of 4xx/5xx
    pub(crate) fn body_len(&self) -> usize {
        let Some(len) = self.content_length() else {
            unreachable!("you should first call self.has_body() to check whether the request has a body then call thos method")
        };

        len.parse().unwrap()
    }
}

impl<'a> Request<'a> {
    pub(crate) fn content_type(&self) -> Option<&&str> {
        self.headers.get("Content-Type")
    }

    pub(crate) fn content_length(&self) -> Option<&&str> {
        self.headers.get("Content-Length")
    }

    pub(crate) fn accept(&self) -> &str {
        self.headers.get("Accept").unwrap_or(&MIME_TYPES)
    }

    pub(super) fn cookie(&self) -> Option<&&str> {
        self.headers.get("Cookie")
    }

    pub(super) fn origin(&self) -> Option<&&str> {
        self.headers.get("origin")
    }

    pub(super) fn access_control_allow_origin(&self) -> Option<&&str> {
        self.headers.get("Access-Control-Allow-Origin")
    }

    pub(super) fn host(&self) -> Option<&&str> {
        self.headers.get("Host")
    }

    pub(super) fn user_agent(&self) -> Option<&&str> {
        self.headers.get("User-Agent")
    }

    pub(super) fn transfer_encoding(&self) -> Option<&&str> {
        self.headers.get("Transfer-Encoding")
    }

    pub(super) fn date(&self) -> Option<&&str> {
        self.headers.get("Date")
    }

    pub fn no_params(&self) -> bool {
        self.request_line.url_params.is_empty()
    }

    pub fn no_headers(&self) -> bool {
        self.headers.is_empty()
    }

    pub fn no_body(&self) -> bool {
        self.body.is_empty()
    }

    pub fn resource(&self) -> &str {
        self.request_line
            .url_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub(crate) fn mime_type(&self) -> Result<&str, RequestErr> {
        let mimetype = match self
            .request_line
            .url_path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
        {
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "json" => "application/json",
            "svg" => "image/svg+xml",
            "jpeg" | "jpg" => "image/jpeg",
            "gif" => "image/gif",
            "avif" => "image/avif",
            "pdf" => "application/pdf",
            _ => "",
        };

        if mimetype.is_empty() {
            return Err(RequestErr::UnsupportedFormat);
        }

        Ok(mimetype)
    }
}
