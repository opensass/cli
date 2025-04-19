use tracing_subscriber::Layer;
use tracing_subscriber::Registry;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{filter, fmt};

pub fn setup_logging() -> anyhow::Result<()> {
    let console_layer = fmt::Layer::new()
        .compact()
        .without_time()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_writer(std::io::stdout)
        .with_filter(filter::LevelFilter::INFO);

    let subscriber = Registry::default().with(console_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
