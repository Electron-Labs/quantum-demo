use nix::sys::socket::{connect, shutdown, socket};
use nix::sys::socket::{AddressFamily, Shutdown, SockAddr, SockFlag, SockType};
use nix::unistd::close;
use protocol_helpers::{recv_loop, recv_u64, send_u64};
use std::os::unix::io::{AsRawFd, RawFd};

use clap::arg;
use clap::command;
use clap::Parser;

pub mod protocol_helpers;

const BUF_MAX_LEN: usize = 8192;
// Maximum number of connection attempts
const MAX_CONNECTION_ATTEMPTS: usize = 5;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ClientArgs {
    /// Enclave CID
    #[arg(long = "cid")]
    pub cid: u32,
    #[arg(long = "port")]
    pub port: u32,
}

struct VsockSocket {
    socket_fd: RawFd,
}

impl VsockSocket {
    fn new(socket_fd: RawFd) -> Self {
        VsockSocket { socket_fd }
    }
}

impl Drop for VsockSocket {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|e| eprintln!("Failed to shut socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close socket: {:?}", e));
    }
}

impl AsRawFd for VsockSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

/// Initiate a connection on an AF_VSOCK socket
fn vsock_connect(cid: u32, port: u32) -> Result<VsockSocket, String> {
    let sockaddr = SockAddr::new_vsock(cid, port);
    let mut err_msg = String::new();

    for i in 0..MAX_CONNECTION_ATTEMPTS {
        let vsocket = VsockSocket::new(
            socket(
                AddressFamily::Vsock,
                SockType::Stream,
                SockFlag::empty(),
                None,
            )
            .map_err(|err| format!("Failed to create the socket: {:?}", err))?,
        );
        match connect(vsocket.as_raw_fd(), &sockaddr) {
            Ok(_) => return Ok(vsocket),
            Err(e) => err_msg = format!("Failed to connect: {}", e),
        }

        // Exponentially backoff before retrying to connect to the socket
        std::thread::sleep(std::time::Duration::from_secs(1 << i));
    }

    Err(err_msg)
}

pub fn get_attestation_doc(args: ClientArgs) -> Result<Vec<u8>, String> {
    let vsocket = vsock_connect(args.cid, args.port)?;
    let fd = vsocket.as_raw_fd();

    send_u64(fd, 0)?;

    let len = recv_u64(fd)?;
    let mut att_doc = [0u8; BUF_MAX_LEN];
    recv_loop(fd, &mut att_doc, len)?;

    Ok(att_doc[..len as usize].to_vec())
}
