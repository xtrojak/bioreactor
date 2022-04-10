use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::thread::sleep;
use std::time::Duration;

const OPEN_CONNECTION: u8 = 0xff;
const MAGIC_NUMBER: u8 = 100; // ASCII(100) = 'd'

const MAJOR_VERSION: u8 = 0xff;
const MINOR_VERSION: u8 = 0xff;

fn main() {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 1234);
    let mut stream = TcpStream::connect(socket).unwrap();
    stream
        .write(&[
            0x00,
            0x06,
            OPEN_CONNECTION,
            MAGIC_NUMBER,
            MAJOR_VERSION,
            MINOR_VERSION,
        ])
        .unwrap();

    loop {
        sleep(Duration::from_secs(10));
    }
}
