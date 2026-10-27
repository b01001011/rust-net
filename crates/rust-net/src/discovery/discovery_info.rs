use crate::discovery::error::{Result};

pub struct DiscoveryInfo {
    st_domain: String,
    st_sub_domain: Option<String>,
    fullname: String,
    hostname: String,
    port: u16,
}

impl DiscoveryInfo {
    pub fn new(
        service_name: &str,
        instance_name: &str,
        hostname: &str,
        port: u16,
    ) -> Result<Self> {
        let (st_domain, st_sub_domain) = 
            if let Some((_, st)) = service_name.rsplit_once("._sub.") {
                (st, Some(service_name))
            } else {
                (service_name, None)
            };

        let st_domain = st_domain.to_string();
        let st_sub_domain = st_sub_domain.map(str::to_string);
        
        let instance_name = escape_instance_name(instance_name);
        let fullname = format!("{instance_name}.{st_domain}");
        let hostname = normalize_hostname(hostname);

        Ok(Self {
            st_domain,
            st_sub_domain,
            fullname,
            hostname,
            port,
        })
    }
}

fn escape_instance_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 10);

    for c in name.chars() {
        match c {
            '.' => {
                result.push('\\');
                result.push('.');
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            _ => result.push(c),
        }
    }
    result
}

fn normalize_hostname(name: &str) -> String {
    if name.ends_with(".local.local.") {
        name[..name.len() - "local.".len()].to_string();    
    }

    name.to_string()
}

