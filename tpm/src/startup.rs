use crate::tpm::*;
use crate::types::*;

pub fn tpm2_startup(tpm: &mut TpmInstance, _args: &StartupArgs) -> Result<(), TpmError> {
    tpm.started = true;
    Err(TpmError {
        rc: TpmRc::CommandCode,
    })
}
