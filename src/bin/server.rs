use std::fs::File;
use std::io::Read;
use std::net::{UdpSocket, SocketAddr};
use tftp::packet::*;

fn handle_read_request(socket: UdpSocket, origin: SocketAddr, buf: Vec<u8>) {

    let req: RequestPacket = bincode::deserialize(&buf).unwrap();

    let mut file = File::open(format!("./{}", req.filename)).unwrap();

    let mut file_bytes_read = 0;
    let mut block_count = 0;

    loop {
        let mut buffer = vec![0; 512];
        file_bytes_read = file.read(&mut buffer).unwrap();

        if file_bytes_read == 0 {
            break;
        }

        let data_packet = DataPacket::new(block_count, &buffer[..file_bytes_read]);

        block_count += 1;

        let serialized = bincode::serialize(&data_packet).unwrap();

        socket.send_to(&serialized, origin);

    }
}

fn handle_write_request(socket: UdpSocket, buf: Vec<u8>) {}

fn handle_ack(buf: Vec<u8>) {}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:5000").unwrap();

    let mut buf = vec![0; 64];

    let (bytes_read, origin) = socket.recv_from(&mut buf).unwrap();

    // Request packets start here
    println!("{}", buf[0]);
    if buf[0] == OpCode::ReadRequest as u8 {
        handle_read_request(socket, origin, buf);
    } else if buf[0] == OpCode::WriteRequest as u8 {
        handle_write_request(socket, buf);
    }
}
