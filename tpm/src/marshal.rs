use crate::types::*;
use core::mem;

pub fn unmarshal_u8(buffer: &[u8], offset: &mut usize) -> Result<u8, TpmError> {
    *offset += 1;

    Ok(buffer[*offset])
}

pub fn unmarshal_u16(buffer: &[u8], offset: &mut usize) -> Result<u16, TpmError> {
    let size = mem::size_of::<u16>();
    let arr = match buffer[*offset..*offset + size].try_into() {
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
    let arr = match buffer[*offset..*offset + size].try_into() {
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
        Ok(su_type_u16) => Ok(StartupType::from(su_type_u16)),
        Err(e) => Err(e),
    }
}

pub fn unmarshal_capability(buffer: &[u8], offset: &mut usize) -> Result<TpmCapability, TpmError> {
    match unmarshal_u32(buffer, offset) {
        Ok(capability) => Ok(TpmCapability::from(capability)),
        Err(e) => Err(e),
    }
}

pub fn unmarshal_startup_args(buffer: &[u8], offset: &mut usize) -> Result<StartupArgs, TpmError> {
    match unmarshal_startup_type(buffer, offset) {
        Ok(su_type) => Ok(StartupArgs { su_type }),
        Err(e) => Err(e),
    }
}

pub fn unmarshal_pt(buffer: &[u8], offset: &mut usize) -> Result<TpmPt, TpmError> {
    match unmarshal_u32(buffer, offset) {
        Ok(pt) => Ok(TpmPt::from(pt)),
        Err(e) => Err(e),
    }
}

pub fn unmarshal_get_capability_args(
    buffer: &[u8],
    offset: &mut usize,
) -> Result<GetCapabilityArgs, TpmError> {
    let mut args = GetCapabilityArgs::default();
    args.cap = match unmarshal_capability(buffer, offset) {
        Ok(cap) => cap,
        Err(e) => return Err(e),
    };

    args.property = match unmarshal_pt(buffer, offset) {
        Ok(prop) => prop,
        Err(e) => return Err(e),
    };

    args.property_count = match unmarshal_u32(buffer, offset) {
        Ok(count) => count,
        Err(e) => return Err(e),
    };

    Ok(args)
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

pub fn marshal_u8(buffer: &mut [u8], val: u8) -> Result<usize, TpmError> {
    if buffer.len() < mem::size_of::<u8>() {
        return Err(TpmError {
            rc: TpmRc::Insufficient,
        });
    }

    buffer[0] = val;

    Ok(mem::size_of::<u8>())
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

pub fn marshal_tpms_tagged_property(
    buffer: &mut [u8],
    val: &TpmsTaggedProperty,
) -> Result<usize, TpmError> {
    let mut offset = match marshal_u32(buffer, val.property as u32) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    offset += match marshal_u32(&mut buffer[offset..], val.val) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    Ok(offset)
}

pub fn marshal_tpmu_capability_data(
    buffer: &mut [u8],
    val: &TpmuCapabilityData,
) -> Result<usize, TpmError> {
    let mut offset = 0;
    match val {
        TpmuCapabilityData::TpmProperties(count, properties) => {
            offset += match marshal_u32(buffer, TpmCapability::TpmProperty as u32) {
                Ok(offset) => offset,
                Err(e) => return Err(e),
            };

            offset += match marshal_u32(&mut buffer[offset..], *count) {
                Ok(offset) => offset,
                Err(e) => return Err(e),
            };

            for i in 0..*count {
                offset += match marshal_tpms_tagged_property(
                    &mut buffer[offset..],
                    &properties[i as usize],
                ) {
                    Ok(offset) => offset,
                    Err(e) => return Err(e),
                };
            }
        }
        TpmuCapabilityData::Unknown => return Err(TpmError { rc: TpmRc::Value }),
    }

    Ok(offset)
}

pub fn marshal_get_capability_response(
    buffer: &mut [u8],
    val: &GetCapabilityResponse,
) -> Result<usize, TpmError> {
    let mut offset = match marshal_u8(buffer, val.more_data as u8) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    offset += match marshal_tpmu_capability_data(&mut buffer[offset..], &val.data) {
        Ok(offset) => offset,
        Err(e) => return Err(e),
    };

    Ok(offset)
}
