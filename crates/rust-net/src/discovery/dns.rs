use std::{
    any::Any,
    fmt,
    time::Instant,
};

pub type DnsRecordBox = Box<dyn DnsRecordExt>;

pub trait DnsRecordExt: fmt::Debug {
    fn any(&self) -> &dyn Any;
    fn get_record(&self) -> &DnsRecord;

    fn expires_soon(&self, now: Instant) -> bool {
        self.get_record().expires_soon(now)
    }
}

#[derive(Debug)]
pub struct DnsEntry {
    pub(crate) name: String,
}

impl DnsEntry {
    const fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}
#[derive(Debug)]
pub struct DnsRecord {
    pub(crate) entry: DnsEntry,
}

impl DnsRecord {
    fn new(name: &str) -> Self {
        Self {
            entry: DnsEntry::new(name.to_string()),
        }
    }

    pub fn expires_soon(&self, now: Instant) -> bool {
        true
    }
}

pub struct DnsInterfaceId {
    pub name: String,
    pub index: u32,
}

#[derive(Debug)]
pub struct DnsPointer {
    record: DnsRecord,
    alias: String,
}

impl DnsPointer {
    pub fn new(name: &str, alias: String) -> Self {
        let record = DnsRecord::new(name); 
        Self {
            record, 
            alias,
        }
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}

impl DnsRecordExt for DnsPointer {
    fn any(&self) -> &dyn Any {
        self
    }

    fn get_record(&self) -> &DnsRecord {
        &self.record
    }
}

