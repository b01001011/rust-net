pub mod discovery;
pub mod ffi;
pub mod rendezvous;
pub mod stun;

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
