
#[derive(Default, Debug, Clone)]
pub struct ServerConfig<'a> {
    server_ip: &'a str,
    pub server_port: u16,
    pub server_name: &'a str,
    pub ephemeral: bool,
    pub group_name: Option<&'a str>,
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

