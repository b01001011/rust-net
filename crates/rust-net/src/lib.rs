pub mod discovery;
pub mod ffi;
pub mod rendezvous;
pub mod stun;

#[cfg(feature = "log")]
use log;

#[cfg(not(feature = "log"))]
#[macro_use]
mod log {
    macro_rules! trace ( ($($tt:tt)*) => {{}} );
    macro_rules! debug ( ($($tt:tt)*) => {{}} );
    macro_rules! info  ( ($($tt:tt)*) => {{}} );
    macro_rules! warn  ( ($($tt:tt)*) => {{}} );
    macro_rules! error ( ($($tt:tt)*) => {{}} );
}

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
