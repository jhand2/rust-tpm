use crate::types::*;
use core::mem;

pub fn unmarshal_u16(buffer: &[u8], offset: &mut usize) -> Result<u16, TpmError> {
    let size = mem::size_of::<u16>();
    let arr = match buffer[*offset..*offset+size].try_into() {
        Ok(arr) => arr,
        Err(_) => {
            return Err(TpmError {
                rc: TpmRc::Insufficient,
            });
        }
    };

    let val = u16::from_be_bytes(arr);
    *offset += size;

    Ok(val)
}

pub fn unmarshal_u32(buffer: &[u8], offset: &mut usize) -> Result<u32, TpmError> {
    let size = mem::size_of::<u32>();
    let arr = match buffer[*offset..*offset+size].try_into() {
        Ok(arr) => arr,
        Err(_) => {
            return Err(TpmError {
                rc: TpmRc::Insufficient,
            });
        }
    };

    let val = u32::from_be_bytes(arr);
    *offset += size;

    Ok(val)
}

pub fn unmarshal_command_code(
    buffer: &[u8],
    offset: &mut usize,
) -> Result<TpmCommandCode, TpmError> {
    match unmarshal_u32(buffer, offset) {
        Ok(code) => {
            let cc = TpmCommandCode::from(code);
            match cc {
                TpmCommandCode::Unknown => Err(TpmError {
                    rc: TpmRc::CommandCode,
                }),
                _ => Ok(cc),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn unmarshal_tag(buffer: &[u8], offset: &mut usize) -> Result<TpmCommandTag, TpmError> {
    match unmarshal_u16(buffer, offset) {
        Ok(tag_u16) => {
            let tag = TpmCommandTag::from(tag_u16);
            match tag {
                TpmCommandTag::Unknown => Err(TpmError { rc: TpmRc::BadTag }),
                _ => Ok(tag),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn unmarshal_startup_type(buffer: &[u8], offset: &mut usize) -> Result<StartupType, TpmError> {
    match unmarshal_u16(buffer, offset) {
        Ok(su_type_u16) => {
            let su_type = StartupType::from(su_type_u16);
            match su_type {
                // TODO: This should be something other than BadTag
                StartupType::Unknown => Err(TpmError { rc: TpmRc::BadTag }),
                _ => Ok(su_type),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn unmarshal_startup_args(buffer: &[u8], offset: &mut usize) -> Result<StartupArgs, TpmError> {
    match unmarshal_startup_type(buffer, offset) {
        Ok(su_type) => Ok(StartupArgs { su_type }),
        Err(e) => Err(e),
    }
}

pub fn unmarshal_command_header(
    buffer: &[u8],
    offset: &mut usize,
) -> Result<CommandHeader, TpmError> {
    let tag = match unmarshal_tag(buffer, offset) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let size = match unmarshal_u32(buffer, offset) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let command_code = match unmarshal_command_code(buffer, offset) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    Ok(CommandHeader {
        tag,
        size: size,
        command_code,
    })
}

pub fn marshal_tag(buffer: &mut [u8], val: TpmCommandTag) -> Result<usize, TpmError> {
    marshal_u16(buffer, val as u16)
}

pub fn marshal_u16(buffer: &mut [u8], val: u16) -> Result<usize, TpmError> {
    if buffer.len() < mem::size_of::<u16>() {
        return Err(TpmError {
            rc: TpmRc::Insufficient,
        });
    }

    let bytes = val.to_be_bytes();
    buffer[0..2].clone_from_slice(&bytes);

    Ok(mem::size_of::<u16>())
}

pub fn marshal_u32(buffer: &mut [u8], val: u32) -> Result<usize, TpmError> {
    if buffer.len() < mem::size_of::<u32>() {
        return Err(TpmError {
            rc: TpmRc::Insufficient,
        });
    }

    let bytes = val.to_be_bytes();
    buffer[0..4].clone_from_slice(&bytes);

    Ok(mem::size_of::<u32>())
}

pub fn marshal_rc(buffer: &mut [u8], val: TpmRc) -> Result<usize, TpmError> {
    marshal_u32(buffer, val as u32)
}

pub fn marshal_response_header(buffer: &mut [u8], val: &ResponseHeader) -> Result<usize, TpmError> {
    let mut offset = match marshal_tag(buffer, val.tag) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    offset = match marshal_u32(&mut buffer[offset..], val.size) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    offset = match marshal_rc(&mut buffer[offset..], val.rc) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    Ok(offset)
}
