#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;
use log::{error, info, trace};

mod cli;
mod error;
mod utils;

fn main() -> Result<(), MyError> {
  let _cli = utils::setup()?;
  trace!("hello thor");
  println!("hello thor");

  Ok(())
}