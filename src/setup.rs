use std::net::TcpListener;

fn build() -> String {
    let mut args = std::env::args();
    args.next();

    let mut host: String = "127.0.0.1".into();
    let mut port: String = "8975".into();
    while let Some(arg) = args.next() {
        match arg.trim() {
            "--port" => {
                port = args
                    .next()
                    .expect("expected a port number, found nothing")
                    .trim()
                    .into();
            }
            "--host" => {
                host = match args
                    .next()
                    .expect("expected host number argument (bidi, in or out), found nothing")
                    .trim()
                {
                    "in" => host,
                    "out" => "192.168.1.138".into(),
                    "bidi" => "0.0.0.0".into(),
                    _ => panic!("bad value"),
                }
            }
            _ => panic!("arg at unexpected position"),
        }
    }

    _ = &port.chars().for_each(|c| assert!(c.is_ascii_digit()));

    host + ":" + &port
}

fn connect(url: &str) -> TcpListener {
    match TcpListener::bind(url) {
        Ok(listener) => listener,
        Err(e) => panic!("couldnt connect to server\n{:?}", e),
    }
}

pub(super) fn init() -> TcpListener {
    let url = build();

    connect(&url)
}
