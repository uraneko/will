// #![feature(buf_read_has_data_left)]
// #![feature(iter_next_chunk)]

// NOTE: the file explorer current dir should be part of the client side internal state
// BUG: cpu usage just spiked all of a sudden after requesting this url
// 'http://127.0.0.1:8975/src/index.html' from the browser, server had been idle for possibly more than half
// an hour prior to that

pub mod files;
mod server;
mod setup;

use server::garcon;
use setup::init;

fn main() {
    // start the server
    let conn = init();

    eprintln!("{:?}", conn);

    // intercept incoming requests
    garcon(conn);
}
