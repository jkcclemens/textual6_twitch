use commands::{Command, HasName};
use commands::onoff::OnOff;

pub struct Subscribers;

impl HasName for Subscribers {
  fn name(&self) -> &str {
    "subscribers"
  }
}

impl OnOff for Subscribers {}

impl Command for Subscribers {}
