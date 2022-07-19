use ctrlc;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::process;
use tpm::marshal;
use tpm::platform;
use tpm::tpm::TpmInstance;
use tpm::types;

const SOCKET_PATH: &str = "/tmp/rust-tpm";

fn handle_request(tpm: &mut TpmInstance, stream: &mut UnixStream) {
    let mut msg_buf = [0u8; types::MAX_MSG_SIZE];
    let mut _size = match stream.read_exact(&mut msg_buf[..types::COMMAND_HDR_SIZE]) {
        Ok(size) => size,
        Err(_) => {
            println!("Failed to read request header");
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

    _size = match stream.read_exact(&mut msg_buf[types::COMMAND_HDR_SIZE..hdr.size as usize]) {
        Ok(size) => size,
        Err(_) => {
            println!("Failed to read request");
            return;
        }
    };

    println!("Executing TPM Command {:#x}", hdr.command_code as u32);

    let mut response: [u8; 4096] = [0; 4096];
    let size = tpm::execute_command(tpm, &mut msg_buf, &mut response);

    match stream.write(&response[..size]) {
        Ok(_) => (),
        Err(e) => println!("Failed to write response: {}", e),
    };
}

fn cleanup() {
    match fs::remove_file(SOCKET_PATH) {
        Ok(_) => (),
        Err(_) => {
            println!("Warning: Unable to unlink {}", SOCKET_PATH);
        }
    }
}

fn print(msg: &str) {
    println!("{}", msg);
}

fn main() -> std::io::Result<()> {
    let socket = Path::new(SOCKET_PATH);
    // Delete old socket if necessary
    if socket.exists() {
        cleanup();
    }

    let listener = UnixListener::bind(socket)?;

    ctrlc::set_handler(move || {
        cleanup();
        process::exit(0);
    })
    .unwrap();

    let host_plat = platform::TpmPlatform { log: print };
    let mut tpm = TpmInstance::new(&host_plat);

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
