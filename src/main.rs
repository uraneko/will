#![feature(path_file_prefix)]
use std::collections::HashMap;
// use std::ffi::OsStr;
use std::io::{BufRead, Write};
use std::io::{Error, ErrorKind};

mod api;
mod file_manager;
mod init;

use init::build;
use meta::get_meta;

fn main() {
    // this is mut because user can provide their own root dir
    // to server in the program args at startup
    let mut server_meta = get_meta();
    build(server_meta);

    println!(
        "serving directory: {} on url:port: {}:{}",
        local_path, dir, port
    );

    let addr = server_meta.listener_uri();
    let listener = std::net::TcpListener::bind(addr).unwrap();

    server_loop(listener);
}
