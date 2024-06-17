#![feature(addr_parse_ascii)]
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize)]
enum ServerDirection {
    In,
    Out,
    All,
}

#[derive(Serialize, Deserialize)]
struct ServerMeta {
    device_addr: SocketAddrV4,
    root_dir: PathBuf,
    server_direction: ServerDirection,
}

fn main() {
    let output = Command::new("hostname").arg("-i").output();

    let device_addr = match output {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("build.rs device_addr get error, {:?}", e);
            Command::new("ip")
                .args(&[
                    "addr", "|", "rg", "192", "|", "string", "replace", "-r", "\"/.*\"", "\"\"",
                    "|", "string", "replace", "inet", "\"\"", "|", "string", "trim",
                ])
                .output()
                .unwrap_or_else(|_| {
                    panic!("could not get ip addr of server device");
                })
        }
    };

    let mut device_addr = device_addr.stdout;

    if device_addr[device_addr.len() - 1] == 10 {
        device_addr.pop();
    }

    // let mut device_addr = String::from_utf8(device_addr).unwrap_or_else(|_| {
    //     panic!("expected output to be string convertible, got a FromUtf8Error");
    // });
    // device_addr.push_str(":5754");
    device_addr.extend(&[58, 53, 55, 52, 53]);

    let mut device_addr = SocketAddrV4::parse_ascii(&device_addr).unwrap();
    device_addr.set_port(5754);

    let root_dir = PathBuf::from("resources/root");
    let meta = ServerMeta {
        device_addr,
        root_dir,
        server_direction: ServerDirection::All,
    };

    let mut file = std::fs::File::create_new("resources/server/server_meta.json").unwrap();
    _ = file.write_all(&serde_json::to_string(&meta).unwrap().into_bytes());
}
