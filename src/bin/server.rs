use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::net::{SocketAddr, UdpSocket};
use tftp::packet::*;

fn handle_read_request(socket: UdpSocket, origin: SocketAddr, buf: Vec<u8>) {
    let req: RequestPacket = bincode::deserialize(&buf).unwrap();

    let mut file = File::open(format!("./{}", req.filename)).unwrap();

    let mut file_bytes_read = 0;
    let mut block = 1;

    // TODO: Check if we can reuse buffers
    let mut buffer = vec![0; 512];
    let mut ack = vec![0; 64];

    loop {
        // Read from file and send to client
        file_bytes_read = file.read(&mut buffer).unwrap();

        if file_bytes_read == 0 {
            break;
        }

        let data_packet = DataPacket::new(block, &buffer[..file_bytes_read]);
        let serialized = bincode::serialize(&data_packet).unwrap();

        socket.send_to(&serialized, origin);

        // Wait for ACK from client
        let result = socket.recv_from(&mut ack);
        match result {
            Ok((bytes_read, origin)) => {},
            Err(e) => {
                println!("Sending block {} again", block);
                continue
            }
        }
        let ack_packet: AckPacket = bincode::deserialize(&ack).unwrap();

        // If ACK packet block is not the same as server, revert the file position
        if ack_packet.block != block {
            println!("Received block: {} but have internal block state of: {}", ack_packet.block, block);
            println!("{}", block as u64 * 512);
            file.seek(SeekFrom::Start(block as u64 * 512));
            continue;
        }
        println!("Received ACK packet successfully");
        block += 1;

    }
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:5000").unwrap();

    let mut buf = vec![0; 64];

    // TODO: Send initial ACK packet
    let (bytes_read, origin) = socket.recv_from(&mut buf).unwrap();

    // Set timeout for user
    socket.set_read_timeout(Some(std::time::Duration::from_secs(2))).unwrap();

    if buf[0] == OpCode::ReadRequest as u8 {
        handle_read_request(socket, origin, buf);
    } else if buf[0] == OpCode::WriteRequest as u8 {
    }

}
