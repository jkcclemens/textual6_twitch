use commands::onoff::OnOff;
use commands::{Command, HasName};

pub struct EmoteOnly;

impl HasName for EmoteOnly {
  fn name(&self) -> &str {
    "emoteonly"
  }
}

impl OnOff for EmoteOnly {}

impl Command for EmoteOnly {}
