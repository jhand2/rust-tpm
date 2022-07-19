#![no_std]

pub mod marshal;
pub mod platform;
pub mod tpm;
pub mod types;

// Command modules
// TODO: This is going to be annoying for every command. Maybe group them?
mod format;
mod get_capability;
mod startup;

use crate::marshal::*;
use crate::types::*;
use tpm::TpmInstance;

pub fn execute_command(tpm: &mut TpmInstance, request: &[u8], response: &mut [u8]) -> usize {
    let mut offset = 0;

    let command_hdr = match unmarshal_command_header(request, &mut offset) {
        Ok(header) => header,
        Err(e) => {
            let response_hdr = ResponseHeader {
                tag: TpmCommandTag::NoSessions,
                size: 0,
                rc: e.rc,
            };
            match marshal_response_header(&mut response[..RESPONSE_HDR_SIZE], &response_hdr) {
                Ok(_) => {
                    return 0usize;
                }
                Err(_) => panic!("Reponse buffer was not big enough for response header"),
            };
        }
    };

    let response_hdr = match tpm.dispatch_command(
        &command_hdr,
        &request[COMMAND_HDR_SIZE..],
        &mut response[RESPONSE_HDR_SIZE..],
    ) {
        Ok(size) => ResponseHeader {
            tag: command_hdr.tag,
            size: size as u32,
            rc: TpmRc::Success,
        },
        Err(e) => ResponseHeader {
            tag: command_hdr.tag,
            size: 0,
            rc: e.rc,
        },
    };

    // If response is too small to hold a header that is a bug in the caller
    // of this function.
    // TODO: Any language construct to help us enforce that a caller can't pass
    // a slice smaller than RESPONSE_HDR_SIZE?
    match marshal_response_header(&mut response[..RESPONSE_HDR_SIZE], &response_hdr) {
        Ok(_) => (),
        Err(_) => panic!("Reponse buffer was not big enough for response header"),
    };

    RESPONSE_HDR_SIZE + response_hdr.size as usize
}
