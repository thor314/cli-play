use anyhow::{anyhow, Result};
use clap::Parser;
use log::trace;

// or:
use crate::cli::subcommand::SubcommandArgs as MyArgs;
// use crate::{cli::MyArgs, error::MyError};
use crate::error::MyError;

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<MyArgs, MyError> {
  dotenv::dotenv().ok();
  // init_tracing();
  let args = MyArgs::parse();
  args.handle();
  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }
  env_logger::builder().filter_level(args.log_level()).build();

  Ok(args)
}
