use std::collections::HashMap;

const MIME_TYPES: &str =
    "text/html, application/xhtml+xml, application/xml;q=0.9, image/webp, */*;q=0.8";

pub(crate) fn parse_request(request: &str) -> Result<Request, RequestErr> {
    if is_head_request(request) {
        parse_head_request(request)
    } else if is_get_request(request) {
        parse_get_request(request)
    } else if is_post_request(request) {
        parse_post_request(request)
    } else {
        Err(RequestErr::UnrecognizedMethod)
    }
}

fn parse_head_request(request: &str) -> Result<Request, RequestErr> {
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
    let mut params = HashMap::new();
    let mut domain = [""; 5];
    parse_url(url, &mut domain, &mut params);

    let lines = lines.collect::<Vec<&str>>();
    let headers = parse_headers(lines);

    Ok(Request::new(
        method,
        version,
        url,
        domain,
        params,
        headers,
        HashMap::new(),
    ))
}

fn parse_get_request(request: &str) -> Result<Request, RequestErr> {
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
    let mut params = HashMap::new();
    let mut domain = [""; 5];
    parse_url(url, &mut domain, &mut params);

    let lines = lines.collect::<Vec<&str>>();
    let headers = parse_headers(lines);

    Ok(Request::new(
        method,
        version,
        url,
        domain,
        params,
        headers,
        HashMap::new(),
    ))
}

fn parse_post_request(request: &str) -> Result<Request, RequestErr> {
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
            version,
            method,
            url,
        });
    }

    // parse the reqline url domain and params
    let mut params = HashMap::new();
    let mut domain = [""; 5];
    parse_url(url, &mut domain, &mut params);

    let mut lines = lines.collect::<Vec<&str>>();
    // TODO: split headers and body
    let headers = parse_headers(lines.drain(..6).collect());

    let body = parse_body(lines);

    Ok(Request::new(
        method, version, url, domain, params, headers, body,
    ))
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
        "HEAD" | "GET" | "POST" => true,
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

fn parse_url(url: &str, dbuf: &mut [&str; 5], pbuf: &mut HashMap<&str, &str>) {
    let [domain, params] = {
        let mut s = url.splitn(1, "?q");
        [s.next(), s.next()]
    };

    if let Some(domain) = domain {
        parse_url_domain(domain, dbuf);
    }

    if let Some(params) = params {
        parse_url_params(params, pbuf);
    }
}

fn parse_url_domain(domain: &str, buf: &mut [&str; 5]) {}

fn parse_url_params(params: &str, buf: &mut HashMap<&str, &str>) {}

fn parse_headers(lines: Vec<&str>) -> HashMap<&str, &str> {
    lines.into_iter().map(|l| parse_header(l)).collect()
}

fn parse_header(line: &str) -> (&str, &str) {
    ("", "")
}

fn parse_body(lines: Vec<&str>) -> HashMap<&str, &str> {
    lines.into_iter().map(|l| parse_field(l)).collect()
}

fn parse_field(line: &str) -> (&str, &str) {
    ("", "")
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
    BadHeaders,
    BadRequestLine {
        method: &'a str,
        url: &'a str,
        version: &'a str,
    },
    BadRequest,
    EmptyRequestLine,
    UnrecognizedMethod,
}
// requestline - headers - optional body

#[derive(Debug)]
struct RequestLine<'a> {
    version: &'a str,
    method: &'a str,
    url: &'a str,
    url_domain: [&'a str; 5],
    url_params: HashMap<&'a str, &'a str>,
}

#[derive(Debug)]
struct Request<'a> {
    request_line: RequestLine<'a>,
    headers: HashMap<&'a str, &'a str>,
    body: HashMap<&'a str, &'a str>,
}

impl<'a> Request<'a> {
    fn new(
        method: &'a str,
        version: &'a str,
        url: &'a str,
        url_domain: [&'a str; 5],
        url_params: HashMap<&'a str, &'a str>,
        headers: HashMap<&'a str, &'a str>,
        body: HashMap<&'a str, &'a str>,
    ) -> Self {
        Self {
            request_line: RequestLine {
                method,
                version,
                url,
                url_domain,
                url_params,
            },
            headers,
            body,
        }
    }

    fn method(&self) -> &str {
        self.request_line.method
    }

    fn version(&self) -> &str {
        self.request_line.version
    }

    fn url(&self) -> &str {
        self.request_line.url
    }

    fn is_http2(&self) -> bool {
        self.request_line.version == "HTTP/2"
    }

    fn is_http1_1(&self) -> bool {
        self.request_line.version == "HTTP/1.1"
    }

    fn is_http3(&self) -> bool {
        self.request_line.version == "HTTP/3"
    }

    fn is_http1_0(&self) -> bool {
        self.request_line.version == "HTTP/1.0"
    }

    // this is needed because e.g.,
    // we could get a post request without body or content type
    pub(crate) fn is_bad(&self) -> bool {
        if self.is_http2()
            && ["Transfer-Encoding", "Upgrade", "Connection"]
                .into_iter()
                .any(|k| self.headers.contains_key(k))
        {
            return true;
        } else if self.is_http1_1() && ["Host"].into_iter().any(|k| !self.headers.contains_key(k)) {
            return true;
        }

        false
    }

    pub(crate) fn how_bad(&self) -> Option<&str> {
        Some("so bad, this string ended up like this")
    }

    // we don't need option because
    // if content type doesnt exist
    // is_bad and how_bad would have cought it
    pub(crate) fn content_type(&self) -> &str {
        self.headers.get("Content-Type").unwrap()
    }

    // this field is obligatory in post requests witha body
    pub(crate) fn content_length(&self) -> Option<&&str> {
        self.headers.get("Content-Length")
    }

    pub(crate) fn accept(&self) -> &str {
        self.headers.get("Accept").unwrap_or(&MIME_TYPES)
    }

    pub fn cookie(&self) -> Option<&&str> {
        self.headers.get("Cookie")
    }

    pub fn origin(&self) -> Option<&&str> {
        self.headers.get("origin")
    }

    pub fn access_control_allow_origin(&self) -> Option<&&str> {
        self.headers.get("Access-Control-Allow-Origin")
    }

    // this field is obligatory
    pub fn host(&self) -> &str {
        self.headers.get("Host").unwrap()
    }

    pub fn user_agent(&self) -> Option<&&str> {
        self.headers.get("User-Agent")
    }

    pub fn transfer_encoding(&self) -> Option<&&str> {
        self.headers.get("Transfer-Encoding")
    }

    pub fn date(&self) -> Option<&&str> {
        self.headers.get("Date")
    }
}
