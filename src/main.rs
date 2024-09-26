#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// NOTE: the file explorer current dir should be part of the client side internal state
// BUG: cpu usage just spiked all of a sudden after requesting this url
// 'http://127.0.0.1:8975/src/index.html' from the browser, server had been idle for possibly more than half
// an hour prior to that

mod files;
mod server;
mod setup;

use server::{garcon, load_cache};
use setup::init;

fn main() {
    // start the server
    let conn = init();

    eprintln!("{:?}", conn);

    let (src_files, app_icons, mut file_icons, mut dirs, status) = load_cache();

    // intercept incoming requests
    garcon(
        conn,
        &src_files,
        &app_icons,
        &mut file_icons,
        &mut dirs,
        &status,
    );
}
