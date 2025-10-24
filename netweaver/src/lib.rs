pub mod analytics;
pub mod cli;
pub mod diagnostics;
pub mod error;
pub mod monitor;
pub mod optimizer;
pub mod plugins;
pub mod scanner;
pub mod security;
pub mod utils;

pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use anyhow::Result;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .with_target(false)
        .with_thread_ids(true)
        .init();
}

pub async fn run() -> Result<()> {
    init_logging();
    cli::run().await
}
