
use std::net::SocketAddr;
use std::io::{Self, Read, Write};

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;

#[derive(Debug)]
struct Server {
    tunnel: tun::Tunnel,
    socket: UdpSocket,
}

impl Future for Server {
    type item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), Error> {
        println!("sending..");

        loop {
            let mut buf = [0; 2048];
            let nb_bytes = try_nb!()
        }
    }
}
