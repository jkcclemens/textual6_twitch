use commands::onoff::OnOff;
use commands::{Command, HasName};

pub struct R9KBeta;

impl HasName for R9KBeta {
  fn name(&self) -> &str {
    "r9kbeta"
  }
}

impl OnOff for R9KBeta {}

impl Command for R9KBeta {}
