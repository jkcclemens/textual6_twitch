use commands::onoff::OnOff;
use commands::{Command, HasName};

pub struct Subscribers;

impl HasName for Subscribers {
  fn name(&self) -> &str {
    "subscribers"
  }
}

impl OnOff for Subscribers {}

impl Command for Subscribers {}
