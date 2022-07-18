use std::os::unix::net::{UnixStream, UnixListener};
use std::io::Read;
use tpm::tpm::TpmInstance;
use tpm::marshal;
use tpm::types;
use std::fs;
use std::process;
use ctrlc;

const SOCKET_PATH: &str = "/tmp/rust-tpm";

fn handle_request(tpm: &mut TpmInstance, stream: &mut UnixStream) {
    let mut msg_buf = [0u8; types::MAX_MSG_SIZE];
    let _size = match stream.read_exact(&mut msg_buf[..types::COMMAND_HDR_SIZE]) {
        Ok(size) => size,
        Err(_) => {
            println!("Failed to read request");
            return;
        }
    };

    let mut offset: usize = 0;
    let hdr = match marshal::unmarshal_command_header(&msg_buf, &mut offset) {
        Ok(hdr) => hdr,
        Err(e) => {
            println!("Unable to parse command header: {}", e);
            return;
        }
    };

    println!("Executing TPM Command {:#x}", hdr.command_code as u32);

    let mut response: [u8; 4096] = [0; 4096];
    tpm::execute_command(tpm, &mut msg_buf, &mut response);
}

fn cleanup() {
    match fs::remove_file(SOCKET_PATH) {
        Ok(_) => (),
        Err(_) => {
            println!("Warning: Unable to unlink {}", SOCKET_PATH);
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind(SOCKET_PATH)?;

    ctrlc::set_handler(move || {
        cleanup();
        process::exit(0);
    }).unwrap();

    let mut tpm = TpmInstance::default();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_request(&mut tpm, &mut stream);
            }
            Err(err) => {
                println!("Failed to open socket: {}", err);
                break;
            }
        }
    }

    cleanup();
    Ok(())
}

