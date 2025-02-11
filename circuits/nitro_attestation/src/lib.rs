use aws_nitro_enclaves_nsm_api::{
    api::{Request, Response},
    driver,
};
use nix::sys::socket::listen as listen_vsock;
use nix::sys::socket::{accept, bind, connect, shutdown, socket};
use nix::sys::socket::{AddressFamily, Shutdown, SockAddr, SockFlag, SockType};
use nix::unistd::close;
use protocol_helpers::{recv_loop, recv_u64, send_loop, send_u64};
use std::os::unix::io::{AsRawFd, RawFd};

use clap::arg;
use clap::command;
use clap::Parser;

pub mod protocol_helpers;

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
const BUF_MAX_LEN: usize = 8192;
// Maximum number of outstanding connections in the socket's
// listen queue
const BACKLOG: usize = 128;
// Maximum number of connection attempts
const MAX_CONNECTION_ATTEMPTS: usize = 5;

#[derive(Debug, Clone)]
pub struct ServerArgs {
    pub port: u32,
}

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

pub fn client(args: ClientArgs) -> Result<Vec<u8>, String> {
    let vsocket = vsock_connect(args.cid, args.port)?;
    let fd = vsocket.as_raw_fd();

    send_u64(fd, 0)?;

    let len = recv_u64(fd)?;
    let mut att_doc = [0u8; BUF_MAX_LEN];
    recv_loop(fd, &mut att_doc, len)?;

    Ok(att_doc[..len as usize].to_vec())
}

fn get_attestation_doc() -> Result<Vec<u8>, String> {
    let nsm_fd = driver::nsm_init();

    let public_key = b"my super secret key".to_vec();
    let hello = b"hello, world!".to_vec();

    let request = Request::Attestation {
        public_key: Some(public_key.into()),
        user_data: Some(hello.into()),
        nonce: None,
    };

    let response = driver::nsm_process_request(nsm_fd, request);
    driver::nsm_exit(nsm_fd);

    match response {
        Response::Attestation { document } => Ok(document),
        _ => Err(format!(
            "nsm driver returned invalid response: {:?}",
            response
        )),
    }
}

/// Accept connections on a certain port and print
/// the received data
pub fn server(args: ServerArgs) -> Result<(), String> {
    let socket_fd = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .map_err(|err| format!("Create socket failed: {:?}", err))?;

    let sockaddr = SockAddr::new_vsock(VMADDR_CID_ANY, args.port);

    bind(socket_fd, &sockaddr).map_err(|err| format!("Bind failed: {:?}", err))?;

    listen_vsock(socket_fd, BACKLOG).map_err(|err| format!("Listen failed: {:?}", err))?;

    loop {
        let fd = accept(socket_fd).map_err(|err| format!("Accept failed: {:?}", err))?;

        let num = recv_u64(fd)?;
        if num == 0 {
            let att_doc = get_attestation_doc()?;
            let len: u64 = att_doc
                .len()
                .try_into()
                .map_err(|err| format!("{:?}", err))?;
            send_u64(fd, len)?;
            send_loop(fd, &att_doc, len)?;
        }
    }
}
