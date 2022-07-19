use crate::format;
use crate::get_capability::*;
use crate::marshal::*;
use crate::platform::*;
use crate::startup::*;
use crate::types::*;
use core::fmt::Arguments;

pub struct TpmInstance {
    pub(crate) started: bool,
    pub(crate) platform: TpmPlatform,
}

impl Default for TpmInstance {
    fn default() -> TpmInstance {
        TpmInstance {
            started: false,
            platform: TpmPlatform::default(),
        }
    }
}

impl TpmInstance {
    pub fn new(platform: &TpmPlatform) -> TpmInstance {
        TpmInstance {
            started: false,
            platform: *platform,
        }
    }
}

impl TpmInstance {
    pub fn log(&self, fmt_args: Arguments) {
        let mut buf = [0u8; 64];
        let s: &str = format::show(&mut buf, fmt_args).unwrap();
        (self.platform.log)(s);
    }

    // Returns number of bytes written to response
    pub fn dispatch_command(
        &mut self,
        command: &CommandHeader,
        param_buffer: &[u8],
        response_buffer: &mut [u8],
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
            }
            TpmCommandCode::GetCapability => {
                let args = match unmarshal_get_capability_args(param_buffer, &mut offset) {
                    Ok(args) => args,
                    Err(e) => return Err(e),
                };

                let response = match tpm2_get_capability(self, &args) {
                    Ok(response) => response,
                    Err(e) => return Err(e),
                };

                let size = match marshal_get_capability_response(response_buffer, &response) {
                    Ok(size) => size,
                    Err(e) => return Err(e),
                };

                Ok(size)
            }
            _ => Err(TpmError {
                rc: TpmRc::CommandCode,
            }),
        }
    }
}
