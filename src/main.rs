#![feature(buf_read_has_data_left)]

mod server;
mod setup;

use server::server;
use setup::init;

fn main() {
    // start the server
    let conn = init();

    eprintln!("{:?}", conn);

    // intercept incoming requests
    server(conn);
}
