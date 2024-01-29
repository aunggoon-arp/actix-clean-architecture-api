use std::{env, net::IpAddr};

use clap::Parser;

lazy_static! {
    pub static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Debug, Parser)]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1", env)]
    pub host: IpAddr,
    #[clap(default_value = "8080", env)]
    pub port: u16,
}
