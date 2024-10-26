// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod common;

use clap::Parser;
use tracing::Level;
use tracing_examples::{models::Model, ExampleClient, ExampleClientExt};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .with_max_level(args.level)
        .init();

    let client = ExampleClient::new("https://api.example.com")?;

    tokio::try_join!(
        client.create_or_update_model(Model::new("foo", "a")),
        client.create_or_update_model(Model::new("bar", "b")),
        client.create_or_update_model(Model::new("baz", "c")),
    )?;

    client.rotate(&args.name, args.secret).await?;

    Ok(())
}

#[derive(Parser)]
struct Args {
    /// Name of the secret to rotate.
    ///
    /// You can include one of [foo, bar, baz], or another name to emit an error.
    name: String,

    /// The new secret.
    ///
    /// If none provided, you will be prompted. You can also pass `-` to accept standard input.
    /// Using the prompt is more secure since the secret will not be entered into command history.
    #[arg(value_parser)]
    secret: common::Secret,

    /// The level to trace.
    #[arg(long, default_value_t = Level::INFO, value_parser)]
    level: Level,
}
