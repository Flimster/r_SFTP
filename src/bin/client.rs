use tftp::packet::*;

use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use std::net::UdpSocket;

const DESTINATION: &str = "/Users/skoli/Documents/Projects/tftp/transferred_files";

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();

    // TODO: Receive first ACK from server
    let rrq_packet = RequestPacket::new(OpCode::ReadRequest as u16, "src/packet.rs", "netascii");

    let rrq_packet = bincode::serialize(&rrq_packet).unwrap();

    socket
        .send_to(&rrq_packet.as_slice(), "127.0.0.1:5000")
        .unwrap();

    let mut file = File::create(format!("{}/{}", DESTINATION, "packet.rs")).unwrap();
    // Initial state has received not blocks
    let mut block = 0;

    loop {
        // 2 bytes + 2 bytes + 8 byte length + 512 byte content
        let mut buf = vec![0; 524];

        let (bytes_read, origin) = socket.recv_from(&mut buf).unwrap();

        if buf[0] == OpCode::Data as u8 {

            println!("Received data packet");

            let data: DataPacket = bincode::deserialize(&buf).unwrap();

            let a = Vec::from(data.data);
            let s = String::from_utf8(a).unwrap();
            if block == data.block {
                println!("Duplicate block");
                continue;
            }

            block = data.block;
            file.write(s.as_bytes()).unwrap();



            println!("Sending back ACK packet");
            // Sending ACK back after writing to file
            let ack_packet = AckPacket::new(block);
            let ack_packet = bincode::serialize(&ack_packet).unwrap();
            socket.send_to(&ack_packet, origin).unwrap();

            block = data.block;

        } else if buf[0] == OpCode::Error as u8 {
            // TODO: Error handling
        }


        if bytes_read != 524 {
            break;
        }
    }
}
