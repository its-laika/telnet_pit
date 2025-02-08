use std::io::{Error, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use telnet_server::read::Read;
use telnet_server::telnet::{Session, State, StateConfig};

const BIND_ADDRESS: &str = "0.0.0.0:23";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    println!("Listening on {}...", BIND_ADDRESS);

    for stream in listener.incoming() {
        thread::spawn(move || {
            if let Ok(stream) = stream {
                let _ = handle_connection(stream);
            };
        });
    }

    Ok(())
}

fn handle_connection(tcp_stream: TcpStream) -> Result<(), Error> {
    let state_config = StateConfig::default();
    let state = State::new(&state_config);

    let remote_ip = tcp_stream
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or("[UNKNOWN]".to_string());

    let mut session = Session::new(state, tcp_stream)?;

    let session_listen = session.clone();
    let _ = thread::spawn(move || session_listen.listen());

    loop {
        session.write_all("Username: ".as_bytes())?;
        let user_name = session.read_line_waiting()?;

        session.write_all("Password: ".as_bytes())?;
        let password = session.read_line_waiting()?;

        session.write_all("Wrong credentials!\r\n".as_bytes())?;

        println!(
            "IP '{}' send username '{}' and password '{}'.",
            &remote_ip,
            user_name.trim(),
            password.trim()
        );
    }
}
