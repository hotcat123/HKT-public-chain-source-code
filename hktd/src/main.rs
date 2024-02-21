mod cli;
mod log_config_watcher;

use self::cli::hktdCmd;
use crate::cli::RunError;
use hkt_primitives::version::{Version, PROTOCOL_VERSION};
use hkt_store::version::DB_VERSION;
use hktcore::get_default_home;
use once_cell::sync::Lazy;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

static hktD_VERSION: &'static str = env!("hktD_VERSION");
static hktD_BUILD: &'static str = env!("hktD_BUILD");
static RUSTC_VERSION: &'static str = env!("hktD_RUSTC_VERSION");

static hktD_VERSION_STRING: Lazy<String> = Lazy::new(|| {
    format!(
        "(release {}) (build {}) (rustc {}) (protocol {}) (db {})",
        hktD_VERSION, hktD_BUILD, RUSTC_VERSION, PROTOCOL_VERSION, DB_VERSION
    )
});

fn hktd_version() -> Version {
    Version {
        version: hktD_VERSION.to_string(),
        build: hktD_BUILD.to_string(),
        rustc_version: RUSTC_VERSION.to_string(),
    }
}

static DEFAULT_HOME: Lazy<PathBuf> = Lazy::new(get_default_home);

#[cfg(feature = "memory_stats")]
#[global_allocator]
static ALLOC: hkt_rust_allocator_proxy::ProxyAllocator<tikv_jemallocator::Jemalloc> =
    hkt_rust_allocator_proxy::ProxyAllocator::new(tikv_jemallocator::Jemalloc);

#[cfg(not(feature = "memory_stats"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> Result<(), RunError> {
    if env::var("RUST_BACKTRACE").is_err() {
        // Enable backtraces on panics by default.
        env::set_var("RUST_BACKTRACE", "1");
    }

    rayon::ThreadPoolBuilder::new()
        .stack_size(8 * 1024 * 1024)
        .build_global()
        .map_err(RunError::RayonInstall)?;

    #[cfg(feature = "memory_stats")]
    ALLOC.set_report_usage_interval(512 << 20).enable_stack_trace(true);
    // We use it to automatically search the for root certificates to perform HTTPS calls
    // (sending telemetry and downloading genesis)
    openssl_probe::init_ssl_cert_env_vars();
    hkt_performance_metrics::process::schedule_printing_performance_stats(Duration::from_secs(60));

    hktdCmd::parse_and_run()
}
