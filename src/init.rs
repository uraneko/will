use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use derive_str_enumify::TryFromStr;

#[derive(Serialize, Deserialize, TryFromStr)]
pub(crate) enum ServerDirection {
    In,
    Out,
    All,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ServerMeta {
    device_addr: SocketAddrV4,
    root_dir: PathBuf,
    server_direction: ServerDirection,
}

impl ServerMeta {
    pub(crate) fn ip_addr(&self) -> SocketAddrV4 {
        match self.server_direction {
            ServerDirection::In => {
                SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), self.device_addr.port())
            }
            ServerDirection::Out => self.device_addr,
            ServerDirection::All => {
                SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), self.device_addr.port())
            }
        }
    }

    pub(crate) fn base_uri(&self) -> String {
        self.ip_addr().ip().to_string()
    }

    pub(crate) fn listener_addr(&self) -> String {
        self.ip_addr().to_string()
    }

    pub(crate) fn dir<'a>(&'a self) -> &'a str {
        &self.root_dir.to_str().unwrap()
    }
}

pub(crate) fn get_meta() -> ServerMeta {
    let meta_file = std::fs::read_to_string("resources/server/server_meta.json").unwrap();

    serde_json::from_str(&meta_file).unwrap()
}

fn stream_args<'a>(args: &mut std::env::Args, builder: &mut ServerMeta) {
    if let Some(arg) = args.next() {
        if &arg[..2] == "--" {
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

    match &flag[..] {
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
             match PathBuf::from_str(&arg) {
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
            let new_direction = ServerDirection::try_from(&arg[..]);
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

pub(crate) fn build(server_meta: &mut ServerMeta) {
    let mut args = std::env::args();
    args.next();

    while !args.is_empty() {
        stream_args(&mut args, server_meta);
    }
}
