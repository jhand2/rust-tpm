use core::fmt::{Display, Formatter, Error};

pub struct TpmError {
    pub rc: TpmRc,
}

impl Display for TpmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "TPM Error {:#04x}", self.rc as u32)
    }
}

// TODO: Fill in all TPM response codes. Should also have some helpers for
// building response codes for different layers.
#[derive(Copy, Clone)]
pub enum TpmRc {
    Success = 0x0,
    BadTag = 0x1E,
    Insufficient = 0x9A,
    Initialize = 0x100,
    CommandCode = 0x143,
}

#[derive(Copy, Clone)]
pub enum TpmCommandCode {
    Startup = 0x144,
    GetCapability = 0x17a,
    Unknown,
}

impl From<u32> for TpmCommandCode {
    fn from(n: u32) -> TpmCommandCode {
        match n {
            0x144 => TpmCommandCode::Startup,
            0x17a => TpmCommandCode::GetCapability,
            _ => TpmCommandCode::Unknown,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TpmCommandTag {
    NoSessions = 0x8001,
    Sessions = 0x8002,
    Unknown,
}

impl From<u16> for TpmCommandTag {
    fn from(n: u16) -> TpmCommandTag {
        match n {
            0x8001 => TpmCommandTag::NoSessions,
            0x8002 => TpmCommandTag::Sessions,
            _ => TpmCommandTag::Unknown,
        }
    }
}

#[derive(Copy, Clone)]
pub enum StartupType {
    Clear = 0x0,
    State = 0x1,
    Unknown,
}

impl From<u16> for StartupType {
    fn from(n: u16) -> StartupType {
        match n {
            0x0 => StartupType::Clear,
            0x1 => StartupType::State,
            _ => StartupType::Unknown,
        }
    }
}

pub const COMMAND_HDR_SIZE: usize = 2 + 4 + 4;
pub const RESPONSE_HDR_SIZE: usize = 2 + 4 + 4;
pub const MAX_MSG_SIZE: usize = 4096;

pub struct CommandHeader {
    pub tag: TpmCommandTag,
    pub size: u32,
    pub command_code: TpmCommandCode,
}

pub struct ResponseHeader {
    pub tag: TpmCommandTag,
    pub size: u32,
    pub rc: TpmRc,
}

pub struct StartupArgs {
    pub su_type: StartupType,
}
