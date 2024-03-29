pub mod api;
pub mod client;
pub mod model;
mod util;
// pub mod util;

pub use client::NacosClient;
pub use model::config::Config;
pub use model::nacos::NacosConfig;

extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
