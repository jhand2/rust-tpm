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
#[repr(u32)]
pub enum TpmRc {
    Success = 0x0,
    BadTag = 0x1E,
    Insufficient = 0x9A,
    Initialize = 0x100,
    CommandCode = 0x143,
}

impl Default for TpmRc {
    fn default() -> Self {TpmRc::Success}
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum TpmCommandCode {
    Startup = 0x144,
    GetCapability = 0x17a,
    Unknown,
}

impl Default for TpmCommandCode {
    fn default() -> Self {TpmCommandCode::Unknown}
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
#[repr(u16)]
pub enum TpmCommandTag {
    NoSessions = 0x8001,
    Sessions = 0x8002,
    Unknown,
}

impl Default for TpmCommandTag {
    fn default() -> Self {TpmCommandTag::Unknown}
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
#[repr(u16)]
pub enum StartupType {
    Clear = 0x0,
    State = 0x1,
    Unknown,
}

impl Default for StartupType {
    fn default() -> Self {StartupType::Unknown}
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

// TODO: Calculate this like mstpm does
const MAX_TPM_PROPERTIES: usize = 8;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TpmPt {
    Manufacturer = 0x105,
    Unknown,
}

impl Default for TpmPt {
    fn default() -> Self {TpmPt::Unknown}
}

impl From<u32> for TpmPt {
    fn from(n: u32) -> TpmPt {
        match n {
            0x105 => TpmPt::Manufacturer,
            _ => TpmPt::Unknown,
        }
    }
}

#[repr(u32)]
pub enum TpmCapability {
    TpmProperty = 0x6,
    Unknown,
}

impl Default for TpmCapability {
    fn default() -> Self {TpmCapability::Unknown}
}

impl From<u32> for TpmCapability {
    fn from(n: u32) -> TpmCapability {
        match n {
            0x6 => TpmCapability::TpmProperty,
            _ => TpmCapability::Unknown,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct TpmsTaggedProperty {
    pub property: TpmPt,
    pub val: u32,
}

#[derive(Clone, Copy)]
pub enum TpmuCapabilities {
    TpmProps{ count: u32, properties: [TpmsTaggedProperty; MAX_TPM_PROPERTIES] },
    Unknown,
}

impl Default for TpmuCapabilities {
    fn default() -> Self {TpmuCapabilities::Unknown}
}

#[derive(Default)]
pub struct TpmsCapabilityData {
    pub cap: TpmCapability,
    pub data: TpmuCapabilities,
}

#[derive(Default)]
pub struct StartupArgs {
    pub su_type: StartupType,
}

#[derive(Default)]
pub struct GetCapabilityArgs {
    pub cap: TpmCapability,
    pub property: u32,
    pub property_count: u32,
}

#[derive(Default)]
pub struct GetCapabilityResponse {
    pub more_data: bool,
    pub data: TpmsCapabilityData,
}
