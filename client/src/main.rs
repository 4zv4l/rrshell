use std::env::args;
use std::net;
use std::io::{stdout,Write,Read};
use rustyline;

fn read_result(conn: &mut net::TcpStream) -> Result<(), std::io::Error> {
    let mut c: [u8; 1] = [0];
    while conn.read_exact(&mut c).is_ok() && c[0] != 0 {
        print!("{}", c[0] as char);
    }
    stdout().flush()
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        return eprintln!("Usage: {} [ip] [port]", args[0]);
    }

    let addr = format!("{}:{}", args[1], args[2]);
    let mut conn = match net::TcpStream::connect(addr) {
        Ok(conn) => conn,
        Err(e) => return eprintln!("{e}"),
    };

    let mut rl = rustyline::DefaultEditor::new().unwrap();
    loop {
        let cmd = match rl.readline("$ ") {
            Ok(cmd) => cmd,
            Err(rustyline::error::ReadlineError::Interrupted) => return,
            Err(e) => return eprintln!("{e}"),
        };

        if cmd.is_empty() { continue; }
        if write!(conn, "{cmd}\0").is_err() { break; }
        if cmd == "exit" || cmd == "stop" { break; }
        if read_result(&mut conn).is_err() { break; }
    }
}
