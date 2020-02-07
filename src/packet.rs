use super::error::TFTPErrorCode;
use serde::{Deserialize, Serialize};

pub const READREQUEST: u8 = 0x01;
pub const WRITEREQUEST: u8 = 0x02;
pub const DATA: u8 = 0x03;
pub const ACKNOWLEDGEMENT: u8 = 0x04;
pub const ERROR: u8 = 0x05;

#[derive(Serialize, Deserialize)]
pub struct RequestPacket {
    pub opcode: u8,
    pub filename: String,
    pub mode: String,
}

impl RequestPacket {
    pub fn new(opcode: u8, filename: &str, mode: &str) -> RequestPacket {
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
    pub opcode: u8,
    pub block: u16,
    pub data: &'a [u8],
}

impl<'a> DataPacket<'a> {
    pub fn new(block: u16, data: &'a [u8]) -> DataPacket {
        DataPacket {
            opcode: DATA,
            block,
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AckPacket {
    pub opcode: u8,
    pub block: u16,
}

impl AckPacket {
    pub fn new(block: u16) -> AckPacket {
        AckPacket {
            opcode: ACKNOWLEDGEMENT,
            block,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorPacket {
    opcode: u8,
    code: TFTPErrorCode,
    msg: String,
}

impl ErrorPacket {
    pub fn new(code: TFTPErrorCode, msg: String) -> ErrorPacket {
        ErrorPacket {
            opcode: ERROR,
            code,
            msg,
        }
    }
}
