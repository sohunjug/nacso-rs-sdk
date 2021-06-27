pub mod api;
pub mod model;
pub mod client;
// pub mod util;

pub use client::NacosClient;
pub use api::config::Config;
pub use model::config::{NacosConfig, ServerConfig};

extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
