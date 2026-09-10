use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_terminal() {
    tracing_subscriber::registry()
        .with(EnvFilter::new("kestrel=info"))
        .with(fmt::layer().with_target(false).without_time())
        .init();
}
