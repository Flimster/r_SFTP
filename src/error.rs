use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TFTPErrorCode {
    NotDefined,
    FileNotFound,
    AccessViolation,
    DiskFull,
    Illegal,
    FileExists,
    NoUser,
}
