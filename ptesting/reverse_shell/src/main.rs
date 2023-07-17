use std::io::{BufRead, BufReader, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
use std::process::{exit, Command};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("[+] Usage:\n\n{} ip port\n\n", args[0]);
        return;
    }
    let port = args[2].parse::<u16>().expect("invalid port number");

    let ip = args[1].as_str().parse::<Ipv4Addr>().unwrap();
    let socket = SocketAddrV4::new(ip, port);

    let tcp_listener = match TcpListener::bind(socket) {
        Ok(l) => l,
        Err(e) => panic!("Bind Error: {:?}", e),
    };

    println!("listening on {}:{}", args[1], args[2]);

    let (mut client_sock, client_addr) = match tcp_listener.accept() {
        Ok(t) => t,
        Err(e) => panic!("Error Accepting Connection: {:?}", e),
    };

    println!(
        "new connection\nsock: {:?}, addr: {:?}",
        client_sock, client_addr
    );

    loop {
        print!("\nCmd: ");
        let mut ip_cmd = String::new();
        io::stdin()
            .read_line(&mut ip_cmd)
            .expect("i need a string!!");
        ip_cmd.push('\0');

        client_sock.write(&ip_cmd.as_bytes());
        let mut buf = Vec::<u8>::new();
        let buf_rdr = BufReader::new(&client_sock);
        buf_rdr.read_until('\0', &mut buf);
    }

    drop(tcp_listener);
}
