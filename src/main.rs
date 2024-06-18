#![feature(path_file_prefix)]
#![feature(exact_size_is_empty)]
// use std::ffi::OsStr;

mod api;
mod endpoints;
mod init;

use api::server_loop;
use init::{build, get_meta};

fn main() {
    // this is mut because user can provide their own root dir
    // to server in the program args at startup
    let mut server_meta = get_meta();
    build(&mut server_meta);

    println!(
        "serving directory: {} on url:port: {}",
        server_meta.dir(),
        server_meta.ip_addr()
    );

    let addr = server_meta.listener_addr();
    let listener = std::net::TcpListener::bind(addr).unwrap();

    server_loop(listener, server_meta.dir());
}
