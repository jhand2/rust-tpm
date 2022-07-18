use crate::marshal::*;
use crate::startup::*;
use crate::get_capability::*;
use crate::types::*;

pub struct TpmInstance {
    pub(crate) started: bool,
}

impl Default for TpmInstance {
    fn default () -> TpmInstance {
        TpmInstance{started: false}
    }
}

impl TpmInstance {
    // Returns number of bytes written to response
    pub fn dispatch_command(
        &mut self,
        command: &CommandHeader,
        param_buffer: &[u8],
        _response: &mut [u8],
    ) -> Result<usize, TpmError> {
        let mut offset = 0;
        match command.command_code {
            TpmCommandCode::Startup => {
                let args = match unmarshal_startup_args(param_buffer, &mut offset) {
                    Ok(args) => args,
                    Err(e) => return Err(e),
                };
                match tpm2_startup(self, &args) {
                    Ok(_) => Ok(0),
                    Err(e) => Err(e),
                }
            },
            TpmCommandCode::GetCapability => {
                let args = match unmarshal_get_capability_args(param_buffer, &mut offset) {
                    Ok(args) => args,
                    Err(e) => return Err(e),
                };
                let mut response = GetCapabilityResponse::default();
                match tpm2_get_capability(self, &args, &mut response) {
                    Ok(_) => Ok(0),
                    Err(e) => Err(e),
                }
            },
            _ => Err(TpmError {
                rc: TpmRc::CommandCode,
            }),
        }
    }
}
