use std::ffi::c_char;
use std::net::SocketAddr;
use std::ptr;

use crate::stun::discover_public_address;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_net_discover_public_address(
    ip_out: *mut c_char,
    ip_size: usize,
    port_out: *mut u16,
) -> i32 {
    if ip_out.is_null() || port_out.is_null() || ip_size == 0 {
        return -1;
    }

    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(_) => return -2,
    };

    let addr: SocketAddr = match runtime.block_on(discover_public_address()) {
        Ok(addr) => addr,
        Err(err) => {
            eprintln!("STUN discovery failed: {err:?}");
            return -3;
        }
    };

    let ip = addr.ip().to_string();
    let bytes = ip.as_bytes();

    if bytes.len() + 1 > ip_size {
        return -1;
    }

    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            ip_out.cast::<u8>(),
            bytes.len(),
        );

        *ip_out.add(bytes.len()) = 0;
        *port_out = addr.port();
    }

    0
}
