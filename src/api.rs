use crate::init::ServerMeta;

enum ResourceKind {
    Icon,
    Dir,
    CSS,
    JS,
    Other(String),
}

enum HTMLMethod {
    GET,
    POST,
}

struct HTMLResponse {
    kind: ResourceKind,
    method: HTMLMethod,
    end_point: String,
    content_type: String,
}

impl HTMLResponse {
    // makes a response from the received request headers
    fn maker(params: LinkedList<String>, method: &str) -> Self {
        let kind = match params.pop_front() {
            "i" => ResourceKind::Icon,
            "d" => ResourceKind::Dir,
            "styles.css" => ResourceKind::CSS,
            "main.js" => ResourceKind::JS,
            other => ResourceKind::Other(other.to_string()),
        };

        let method = match &method.to_uppercase() {
            "GET" => HTMLMethod::GET,
            "POST" => HTMLMethod::POST,
        };

        let end_point = params.pop_front();

        let content_type = HTMLResponse::content_type(&kind);

        Self {
            kind,
            method,
            end_point,
            content_type,
        }
    }

    fn content_type(kind: &ResourceKind) -> String {
        match kind {
            ResourceKind::Icon => "text/xml+svg".to_string(),
            ResourceKind::Dir => "text/html".to_string(),
            ResourceKind::CSS => "text/css".to_string(),
            ResourceKind::JS => "application/js".to_string(),
            ResourceKind::Other(other) => other,
        }
    }

    fn header(&self) -> String {
        let header = String::from("http/1.1 ");
        match &self.content_type {
            "text/html" => header += "200 OK",
            _ => header += "201 OK",
        };
        header +=
            "\r\nAccept-Range:bytes\r\nContent-Type:" + content_type + "; charset=utf-8\r\n\r\n";

        header
    }

    fn body(&self, params: &mut LinkedList<String>) -> String {
        match self.kind {
            ResourceKind::Icon => HTMLResponse::resolve_icon(params),
            ResourceKind::CSS => HTMLResponse::resolve_css(),
            ResourceKind::JS => HTMLResponse::resolve_js(),
            ResourceKind::Dir => HTMLResponse::handle_dir(params),
            ResourceKind::Other(o) => todo!(),
        }
        .to_string()
    }

    fn resolve_dir(params: &mut LinkedList<String>) -> Result<&str, std::io::Error> {
        let dir = match params.pop_back() {
            Some(leaf) => leaf,
            None => return Err("unresolvable dir uri"),
        };

        match dir {
            "default" => Ok("/resources/root"),
            random_dir => Ok(random_dir),
        }
    }

    fn resolve_icon(params: &mut LinkedList<String>) -> &str {
        let icon = match params.pop_back() {
            Some(icon) => icon,
            None => return Err("unresolvable icon uri"),
        };

        &("/resources/icons/".to_string() + icon + ".svg")[..]
    }
}

pub(crate) fn server_loop(listener: TcpListener) {
    for mut stream in listener.incoming().flatten() {
        let mut reader = std::io::BufReader::new(&mut stream);
        eprintln!("{:?}", reader);
        // get resource
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
            // TODO: security for received resource var
            // make sure received resource is not malicious string
            ["GET", uri, "HTTP/1.1"] => handle_get(uri),
            ["POST", uri, "HTTP/1.1"] => handle_post(uri),
            _ => todo!(),
        }
    }
}

fn parse_uri<'a>(uri: &str) -> LinkedList<&'a str> {
    let mut chunks = uri.split("/").filter(|c| c != "");

    let mut params = LinkedList::with_capacity(chunks.len());
    while let Some(chunk) = chunks.next() {
        params.push_back(chunk);
    }

    params
}

fn handle_get(uri: &str) {
    let params = parse_uri(uri);
    loop {
        // NOTE: we're not doing anything with this stuff
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        print!("{}", &line[..]);
    }
    println!("\r\n\r\n\r\n");

    // /i/home <- this is uri for getting home.svg icon from resources/icons
    // pop_back

    let res = HTMLResponse::maker(params, "get");

    stream.write_all(res.header()).unwrap();
    let res = std::fs::read(path); // end point callback data goes here
}

fn handle_post(uri: &str) {}
