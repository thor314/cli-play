use anyhow::{anyhow, Result};
use clap::Parser;
use log::trace;

use crate::{cli::MyArgs, error::MyError};
// or:
// use crate::cli::subcommand::SubcommandArgs;

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<MyArgs, MyError> {
  dotenv::dotenv().ok();
  // init_tracing();
  let args = MyArgs::parse();
  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }
  env_logger::builder().filter_level(args.log_level()).build();

  Ok(args)
}
