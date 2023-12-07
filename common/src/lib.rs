pub use eyre::Context;
pub use eyre::Result;
pub use tracing::{debug, info};

use std::env::current_dir;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub fn setup() -> Result<String> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .compact()
        .with_level(false)
        .with_line_number(true)
        .finish();

    subscriber.try_init()?;
    debug!("Tracing init");

    let path = current_dir()?;
    let input_file = path.join("input.txt");

    let input = std::fs::read_to_string(input_file).context("Reading input file input.txt")?;

    Ok(input)
}
