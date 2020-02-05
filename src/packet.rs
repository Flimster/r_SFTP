use super::error::TFTPErrorCode;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub enum OpCode {
    ReadRequest = 0x01,
    WriteRequest = 0x02,
    Data = 0x03,
    Acknowledgement = 0x04,
    Error = 0x05,
}

#[derive(Serialize, Deserialize)]
pub struct RequestPacket {
    pub opcode: u16,
    pub filename: String,
    pub mode: String,
}

impl RequestPacket {
    pub fn new(opcode: u16, filename: &str, mode: &str) -> RequestPacket {
        let filename = String::from(filename);
        let mode = String::from(mode);
        RequestPacket {
            opcode,
            filename,
            mode,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataPacket<'a> {
    pub opcode: u16,
    pub block: u16,
    pub data: &'a[u8],
}

impl<'a> DataPacket<'a> {
    pub fn new(block: u16, data: &'a[u8] ) -> DataPacket {
        DataPacket {
            opcode: OpCode::Data as u16,
            block,
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AckPacket {
    opcode: u16,
    block: u16,
}

impl AckPacket {
    pub fn new(block: u16) -> AckPacket {
        AckPacket {
            opcode: OpCode::Acknowledgement as u16,
            block,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorPacket {
    opcode: u16,
    code: TFTPErrorCode,
    msg: String,
}

impl ErrorPacket {
    pub fn new(code: TFTPErrorCode, msg: String) -> ErrorPacket {
        ErrorPacket {
            opcode: OpCode::Error as u16,
            code,
            msg,
        }
    }
}
