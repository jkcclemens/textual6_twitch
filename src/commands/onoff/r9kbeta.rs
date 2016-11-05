use commands::{Command, HasName};
use commands::onoff::OnOff;

pub struct R9KBeta;

impl HasName for R9KBeta {
  fn name(&self) -> &str {
    "r9kbeta"
  }
}

impl OnOff for R9KBeta {}

impl Command for R9KBeta {}
