use std::collections::HashMap;

#[derive(Clone)]
pub struct NacosConfig<'a> {
    scheme: &'a str,
    auth: bool,
    nacos_ip: &'a str,
    nacos_port: u32,
    nacos_user: &'a str,
    nacos_pass: &'a str,
}

#[derive(Default, Debug, Clone)]
pub struct ServerConfig<'a> {
    server_ip: &'a str,
    pub server_port: u16,
    pub server_name: &'a str,
    pub ephemeral: bool,
    pub group_name: Option<&'a str>,
}

#[derive(Default, Debug, Clone)]
pub struct DeployConfig<'a> {
    data_id: &'a str,
    group: &'a str,
    tenant: Option<&'a str>,
}

impl<'a> Default for NacosConfig<'a> {
    fn default() -> Self {
        Self {
            scheme: "http",
            auth: false,
            nacos_ip: "127.0.0.1",
            nacos_port: 8848,
            nacos_user: "nacos",
            nacos_pass: "nacos",
        }
    }
}

impl<'a> NacosConfig<'a> {
    pub fn new(scheme: &'a str, nacos_ip: &'a str, nacos_port: u32) -> Self {
        Self {
            scheme,
            auth: false,
            nacos_ip,
            nacos_port,
            nacos_user: "nacos",
            nacos_pass: "nacos",
        }
    }

    pub fn new_with_auth(scheme: &'a str, nacos_ip: &'a str, nacos_port: u32, nacos_user: &'a str, nacos_pass: &'a str) -> Self {
        Self {
            scheme,
            auth: true,
            nacos_ip,
            nacos_port,
            nacos_user,
            nacos_pass,
        }
    }

    pub fn exchange(&mut self, ex: Self) -> Self {
        let prev = self.clone();
        self.scheme = ex.scheme;
        self.auth = ex.auth;
        self.nacos_ip = ex.nacos_ip;
        self.nacos_port = ex.nacos_port;
        self.nacos_user = ex.nacos_user;
        self.nacos_pass = ex.nacos_pass;
        prev
    }

    pub fn addr(&self, target: &str) -> String {
        let sub_path = if target.starts_with('/')
        { target.to_string() } else { format!("/{}", target) };
        format!(
            "{}://{}:{}/nacos{}",
            self.scheme, self.nacos_ip, self.nacos_port, sub_path
        )
    }
}


impl<'a> ServerConfig<'a> {
    pub fn set_server_port(&mut self, server_port: u16) {
        self.server_port = server_port;
    }
    pub fn set_server_name(&mut self, server_name: &'a str) {
        self.server_name = server_name;
    }
    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }
    pub fn set_group_name(&mut self, group_name: Option<&'a str>) {
        self.group_name = group_name;
    }
}

impl<'a> ServerConfig<'a> {
    pub fn server_ip(&self) -> &str { &self.server_ip }
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    pub fn server_name(&self) -> &str {
        &self.server_name
    }
    pub fn ephemeral(&self) -> bool {
        self.ephemeral
    }
    pub fn group_name(&self) -> &Option<&'a str> {
        &self.group_name
    }
}

impl<'a> ServerConfig<'a> {
    pub fn new(server_ip: &'a str, server_port: u16, server_name: &'a str) -> Self {
        Self {
            server_ip,
            server_port,
            server_name,
            ephemeral: false,
            group_name: None,
        }
    }

    pub(crate) fn init_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        map.insert("ip".to_string(), self.server_ip().to_string());
        map.insert("port".to_string(), self.server_port().to_string());
        map.insert("serviceName".to_string(), self.server_name().to_string());
        if self.ephemeral {
            map.insert("ephemeral".to_string(), true.to_string());
        }
        if let Some(s) = &self.group_name {
            map.insert("groupName".to_string(), s.to_string());
        }
        map
    }
}

impl<'a> DeployConfig<'a> {
    pub fn new(data_id: &'a str, group: &'a str, tenant: Option<&'a str>) -> Self {
        Self {
            data_id,
            group,
            tenant,
        }
    }

    pub fn init_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::<String, String>::new();
        map.insert("dataId".to_string(), self.data_id.to_string());
        map.insert("group".to_string(), self.group.to_string());
        if let Some(tenant) = &self.tenant {
            map.insert("tenant".to_string(), tenant.to_string());
        };
        map
    }

    pub fn data_id(&self) -> &str {
        &self.data_id
    }
    pub fn group(&self) -> &str {
        &self.group
    }
    pub fn tenant(&self) -> &Option<&'a str> {
        &self.tenant
    }
}
