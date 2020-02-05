use tftp::packet::*;

use std::fs::File;
use std::net::UdpSocket;
use std::io::Write;
use std::borrow::Borrow;

const DESTINATION: &str = "/Users/skoli/Documents/Projects/tftp/transferred_files";

fn handle_data(packet: Vec<u8>) {
    let data: DataPacket = bincode::deserialize(&packet).unwrap();

    let a = Vec::from(data.data);

    let s = String::from_utf8(a).unwrap();

    println!("{}", s);
}

fn handle_error(packet: ErrorPacket) {}

fn handle_ack(packet: AckPacket) {}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();

    let rrq_packet = RequestPacket::new(OpCode::ReadRequest as u16, "src/lib.rs", "netascii");

    let serialized_packet = bincode::serialize(&rrq_packet).unwrap();

    socket
        .send_to(serialized_packet.as_slice(), "127.0.0.1:5000")
        .unwrap();

    let mut file = File::create(format!("{}/{}", DESTINATION, "tmp.rs")).unwrap();
    // 2 bytes + 2 bytes + 8 byte length + 512 byte content
    loop {
        let mut buf = vec![0; 524];

        let (bytes_read, _) = socket.recv_from(&mut buf).unwrap();

        println!("{:#?}", buf);
        println!("{}", bytes_read);


        if buf[0] == 0x03 {
            let data: DataPacket = bincode::deserialize(&buf).unwrap();
            let a = Vec::from(data.data);
            let s = String::from_utf8(a).unwrap();
            file.write(s.as_bytes());
//            handle_data(buf);
        }

        if bytes_read != 524 {
            break;
        }
    }
}
