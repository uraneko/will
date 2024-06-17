use std::net::SocketV4IpAddr;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
enum ServerDirection {
    In,
    Out,
    All,
}

#[derive(Serialize, Deserialize)]
struct ServerMeta {
    device_addr: SocketV4IpAddr,
    root_dir: PathBuf,
    server_direction: ServerDirection,
}

impl ServerMeta {
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

fn get_meta() -> ServerMeta {
    let meta_file = std::fs::read_file("resources/server/server_meta.json").unwrap();

    serde_json::from_str(meta_file).unwrap()
}

fn stream_args<'a>(args: &mut std::env::Args, builder: &mut ServerMeta) {
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

fn mutate_builder<'a>(builder: &mut ServerMeta, flag: String, arg: Option<String>) {
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

pub(crate) fn build(server_meta: ServerMeta) {
    let mut args = std::env::args();
    args.next();

    while !args.is_empty() {
        stream_args(&mut args, builder);
    }
}
