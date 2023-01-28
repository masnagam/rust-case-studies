use tracing_subscriber::EnvFilter;

fn main() {
    println!("RUST_LOG='info , spaces_in_rust_log::x=debug'");
    std::env::set_var("RUST_LOG", "info, spaces_in_rust_log::x=debug");

    if cfg!(feature = "env_logger") {
        println!("Use env_logger");
        env_logger::init();
    } else {
        println!("Use tracing_subscriber::EnvFilter");
        tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    }

    log::info!("");
    log::debug!("");
    x::logging();
}

mod x {
    pub fn logging() {
        log::info!("");
        log::debug!("");
    }
}
