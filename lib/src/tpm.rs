use crate::types::*;
use crate::marshal::*;
use crate::startup::*;

pub struct TpmInstance {
    pub (crate) started: bool,
}

impl TpmInstance {

    // Returns number of bytes written to response
    pub fn dispatch_command(&mut self, command: &CommandHeader, param_buffer: &[u8],
                        response: &mut [u8]) -> Result<usize, TpmError> {
        match command.command_code {
            TpmCommandCode::Startup => {
                let mut offset = 0;
                let args = match unmarshal_startup_args(param_buffer, &mut offset) {
                    Ok(args) => args,
                    Err(e) => return Err(e)
                };
                match tpm2_startup(self, &args) {
                    Ok(_) => Ok(0),
                    Err(e) => Err(e)
                }
            },
            _ => Err(TpmError{rc: TpmRc::CommandCode})
        }
    }
}
