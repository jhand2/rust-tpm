use crate::types::*;
use crate::tpm::*;

pub fn tpm2_startup(tpm: &mut TpmInstance, args: &StartupArgs) -> Result<(), TpmError> {
    tpm.started = true;
    Err(TpmError{rc: TpmRc::CommandCode})
}
