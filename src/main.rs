#![feature(path_file_prefix)]
use std::collections::HashMap;
// use std::ffi::OsStr;
use std::io::{BufRead, Write};
use std::io::{Error, ErrorKind};

mod api;
mod builder;
mod file_manager;
mod init;

use init::build;
use meta::get_meta;

const RES_JS: &str =
    "HTTP/1.1 201 OK\r\nAccept-Ranges:bytes\r\nContent-Type:text/javascript; charset=utf-8\r\n\r\n";
const RES_CSS: &str =
    "HTTP/1.1 201 OK\r\nAccept-Range:bytes\r\nContent-Type:text/css, charset=utf-8\r\n\r\n";
const RES_HTML: &str = "HTTP/1.1 200 OK Content-Type:text/html; charset=utf-8\r\n\r\n";
const RES_GENERAL: &str = "HTTP/1.1 200 OK\r\n\r\n";

fn main() {
    // this is mut because user can provide their own root dir
    // to server in the program args at startup
    let mut server_builder = get_meta();
    build(server_builder);

    println!(
        "serving directory: {} on url:port: {}:{}",
        local_path, dir, port
    );
    let addr = server_builder.listener_uri();

    let listener = std::net::TcpListener::bind(addr).unwrap();
}
