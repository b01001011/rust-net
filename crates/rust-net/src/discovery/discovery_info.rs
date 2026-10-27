use crate::discovery::error::{Result};

pub struct DiscoveryInfo {
    st_domain: String,
    st_sub_domain: Option<String>,
    fullname: String,
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

        let fullname = format!(".{st_domain}");

        Ok(Self {
            st_domain,
            st_sub_domain,
            fullname,
            port,
        })
    }
}

