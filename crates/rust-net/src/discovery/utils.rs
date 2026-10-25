use if_addrs::{Interface};

pub(super) fn local_if_addrs() -> Vec<Interface> {
    let mut interfaces = if_addrs::get_if_addrs().unwrap_or_default();
    interfaces.retain(|i| i.is_oper_up() && !i.is_p2p());
    interfaces
}

