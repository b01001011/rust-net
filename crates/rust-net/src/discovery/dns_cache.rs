#[cfg(feature = "logging")]
use crate::log::{debug, trace};
use crate::discovery::dns::{
    DnsInterfaceId,
    DnsRecordBox,
};
use std::{
    collections::{HashMap, HashSet},
};

pub(crate) struct DnsRecordInterface {
    pub(crate) record: DnsRecordBox,
    pub(crate) src_interface: DnsInterfaceId,
}

pub(crate) struct DnsCache {
    ptr: HashMap<String, Vec<DnsRecordInterface>>,
}

impl DnsCache {
    pub(crate) fn new() -> Self {
        Self {
            ptr: HashMap::new(),
        }
    }

    pub(crate) fn get_ptr(&self, service_name: &str) -> Option<&Vec<DnsRecordInterface>> {
        self.ptr.get(service_name)
    }
}
