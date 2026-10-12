pub mod rendezvous;
pub mod stun;

// pub use rendezvous::server::RendezvousServer;
// pub use stun::discovery::discover_public_address;

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
