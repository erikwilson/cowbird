use super::*;
use std::net::{TcpListener, TcpStream, UdpSocket};

#[test]
fn test_send_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = &listener.local_addr().unwrap();
    send(addr, "hello tcp", &Protocol::TCP);
    let (mut stream, _) = listener.accept().unwrap();
    let mut data = String::new();
    stream.read_to_string(&mut data);
    assert_eq!(data, "hello tcp");
}

#[test]
fn test_send_udp() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let addr = &socket.local_addr().unwrap();
    send(addr, "hello udp", &Protocol::UDP);
    let mut buf = [0; 10];
    let (len, _) = socket.recv_from(&mut buf).unwrap();
    let data = String::from_utf8(buf[..len].to_vec()).unwrap();
    assert_eq!(data, "hello udp");
}
