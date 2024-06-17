use std::net::SocketV4IpAddr;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
enum ServerDirection {
    In,
    Out,
    All,
}

#[derive(Serialize, Deserialize)]
struct ServerBuilder {
    device_addr: SocketV4IpAddr,
    root_dir: PathBuf,
    server_direction: ServerDirection,
}

impl ServerBuilder {
    fn to_ip(&self) -> &SocketV4IpAddr {
        match self.server_direction {
            ServerDirection::In => {
                SocketV4IpAddr::new(Ipv4Addr::new(127, 0, 0, 1), self.device_addr.port())
            }
            ServerDirection::Out => self.device_addr,
            ServerDirection::All => {
                SocketV4IpAddr::new(Ipv4Addr::new(0, 0, 0, 0), self.device_addr.port())
            }
        }
    }

    pub(crate) fn base_uri<'a>(&self) -> &'a str {
        self.to_ip().ip().to_string().as_str()
    }

    pub(crate) fn listener_addr<'a>(&self) -> &'a str {
        self.to_ip().to_string().as_str()
    }
}

fn get_meta() -> ServerBuilder {
    let meta_file = std::fs::read_file("resources/server/server_meta.json").unwrap();

    serde_json::from_str(meta_file).unwrap()
}

fn stream_args<'a>(args: &mut std::env::Args, builder: &mut ServerBuilder) {
    if let Some(arg) = args.next() {
        if arg[..2] == "--" {
            if let Some(val) = args.next() {
                mutate_builder(builder, arg, Some(val));
            } else {
                mutate_builder(builder, arg, None);
            }
        }
    }
}

fn mutate_builder<'a>(builder: &mut ServerBuilder, flag: String, arg: Option<String>) {
    // there is no flag that takes no arg yet
    if arg.is_none() {
        return;
    }
    let arg = arg.unwrap();

    match &flag {
        "--port" => {
            let port_num = match arg.parse() {
                Ok(int) => int,
                Err(e) => {
                    eprintln!(
                        "bad port number provided, failed to change ip address port\n{:?}",
                        e
                    );
                    return;
                }
            };
            builder.device_addr.set_port(port_num);
        }
        "--root-dir" => {
            let new_dir = match PathBuf::from_str(arg) {
                Ok(pb) => match pb.is_dir() {
                    true => builder.root_dir = pb,
                    false => {
                        eprintln!(
                            "the path provided is not a directory, failed to change server root dir",
                        );
                        return;
                    }
                },
                Err(e) => {
                    eprintln!(
                        "invalid root dir path provided, failed to change server root dir\n{:?}",
                        e
                    );
                    return;
                }
            };
        }
        "--server-direction" => {
            let new_direction = ServerDirection::str_enumify(arg).unwrap();
            match new_direction {
                // TODO: change str_enumify to return a result of the enum variant
                // rather than panic
                Ok(en_var) => builder.server_direction = en_var,
                Err(e) => {
                    eprintln!("user given server direction is not a proper ServerDirection enum variant use one of: All, In or Out\n{:?}", e);
                    return;
                }
            }
        }
        _ => eprintln!("unrecognized argument flag provided, try --help for a list of what arguments you can pass"),
    }
}

pub(crate) fn build(server_builder: ServerBuilder) -> HashMap<String, String> {
    let mut args = std::env::args();
    args.next();

    while !args.is_empty() {
        stream_args(&mut args, builder);
    }
}

fn server_loop(listener: TcpListener) {
    for mut stream in listener.incoming().flatten() {
        let mut reader = std::io::BufReader::new(&mut stream);
        eprintln!("{:?}", reader);
        // get resource
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
            // TODO: security for received resource var
            // make sure received resource is not malicious string
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line.trim().is_empty() {
                        break;
                    }
                    print!("{}", &line[..]);
                }
                println!("\r\n\r\n\r\n");
                let mut path = std::path::PathBuf::new();
                path.push(&local_path[..]);
                path.push(resource.trim_start_matches("/"));
                if resource.ends_with('/') {
                    path.push("index.html");
                }
                // TODO: js not working
                // TODO: if asked for file is outside of src abort all
                // make sure there is no (../), (/some_path) and the likes involved
                eprintln!(
                    "the path to the html is: {:?}\n{:?}",
                    &path,
                    &path.extension()
                );

                let res_header = match path.extension().unwrap().to_str() {
                    Some("js") => RES_JS,
                    Some("css") => RES_CSS,
                    Some("html") => RES_HTML,
                    _ => RES_GENERAL,
                };
                println!("headers::: {}", res_header);
                stream.write_all(res_header.as_bytes()).unwrap();
                let res = std::fs::read(path);
                match res {
                    Ok(_) => stream.write_all(&res.unwrap()).unwrap(),
                    Err(err) => match err.kind() {
                        ErrorKind::NotFound => {
                            eprintln!("bypassing response error, probably false positive of favicon.ico not found");
                        }
                        _ => {
                            Error::other(err);
                        }
                    },
                }
            }
            _ => {}
        }
        // using loop

        //  using while
        // let mut line = String::new();
        // reader.read_line(&mut line).unwrap();
        // while !line.trim().is_empty() {
        //     print!("{}", &line[..]);
        //     line = String::new();
        //     reader.read_line(&mut line).unwrap();
        // }
    }
}
