use tftp::packet::*;

use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use std::net::UdpSocket;

const DESTINATION: &str = "/Users/skoli/Documents/Projects/tftp/transferred_files";

struct Client {
    file: File,
    filename: String,
    socket: UdpSocket,
    block: u16
}

impl Client {
    pub fn new(filename: &str, server_addr: &str) -> Client {
        let file = File::create(format!("{}/{}", DESTINATION, "tmp.rs")).unwrap();
        let filename = String::from(filename);
        let socket = UdpSocket::bind(server_addr).unwrap();

        Client {
            file,
            filename,
            socket,
            block: 0
        }
    }

    pub fn send_first_packet(&mut self) {
        let rrq_packet = RequestPacket::new(READREQUEST, self.filename.as_str(), "netascii");
        let serialized_packet = bincode::serialize(&rrq_packet).unwrap();

        self.socket
            .send_to(&serialized_packet.as_slice(), "127.0.0.1:5000")
            .unwrap();
    }

    pub fn start_receiving(&mut self) {
        let mut buf = vec![0; 523];
        loop {
            // 1 byte + 2 bytes + 8 byte length + 512 byte content
            let (bytes_read, origin) = self.socket.recv_from(&mut buf).unwrap();
            // Checking package type
            // TODO: Maybe add more opcodes
            match buf[0] {
                DATA => {
                    self.handle_data_packet(&buf.as_slice()[..bytes_read]);
                }
                ERROR => {
                    eprintln!("Error packet received, quitting...");
                }
                _ => {
                    eprintln!("Unknown opcode received")
                }
            }

            // Final data block
            if bytes_read != 523 {
                break;
            }
        }
    }

    fn handle_data_packet(&mut self, buffer: &[u8]) {
        println!("Received data packet");


        let data: DataPacket = bincode::deserialize(buffer).unwrap();

        if data.block == self.block {
            println!("Duplicated block");
            return;
        }

        self.file.write(data.data);

        self.block = data.block;

        let ack_packet = AckPacket::new(self.block);
        let ack_packet = bincode::serialize(&ack_packet).unwrap();
        // TODO: Change origin, add as a field
        self.socket.send_to(&ack_packet, "127.0.0.1:5000").unwrap();

    }
}

fn main() {
    let mut client = Client::new("src/packet.rs", "127.0.0.1:3000");
    client.send_first_packet();
    client.start_receiving()
//    let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();
//
//    // TODO: Receive first ACK from server
//    let rrq_packet = RequestPacket::new(OpCode::ReadRequest as u16, "src/packet.rs", "netascii");
//
//    let rrq_packet = bincode::serialize(&rrq_packet).unwrap();
//
//    socket
//        .send_to(&rrq_packet.as_slice(), "127.0.0.1:5000")
//        .unwrap();
//
//    let mut file = File::create(format!("{}/{}", DESTINATION, "packet.rs")).unwrap();
//    // Initial state has received not blocks
//    let mut block = 0;
//
//    loop {
//        // 2 bytes + 2 bytes + 8 byte length + 512 byte content
//        let mut buf = vec![0; 524];
//
//        let (bytes_read, origin) = socket.recv_from(&mut buf).unwrap();
//
//        if buf[0] == OpCode::Data as u8 {
//
//            println!("Received data packet");
//
//            let data: DataPacket = bincode::deserialize(&buf).unwrap();
//
//            let a = Vec::from(data.data);
//            let s = String::from_utf8(a).unwrap();
//            if block == data.block {
//                println!("Duplicate block");
//                continue;
//            }
//
//            block = data.block;
//            file.write(s.as_bytes()).unwrap();
//
//
//
//            println!("Sending back ACK packet");
//            // Sending ACK back after writing to file
//            let ack_packet = AckPacket::new(block);
//            let ack_packet = bincode::serialize(&ack_packet).unwrap();
//            socket.send_to(&ack_packet, origin).unwrap();
//
//            block = data.block;
//
//        } else if buf[0] == OpCode::Error as u8 {
//            // TODO: Error handling
//        }
//
//
//        if bytes_read != 524 {
//            break;
//        }
//    }
}
