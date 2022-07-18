use crate::tpm::*;
use crate::types::*;

pub fn tpm2_get_capability(_tpm: &mut TpmInstance,
        _args: &GetCapabilityArgs, _response: &mut GetCapabilityResponse) -> Result<(), TpmError> {
    Err(TpmError {
        rc: TpmRc::CommandCode,
    })
}
