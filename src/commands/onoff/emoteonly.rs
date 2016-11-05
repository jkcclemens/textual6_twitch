use commands::{Command, HasName};
use commands::onoff::OnOff;

pub struct EmoteOnly;

impl HasName for EmoteOnly {
  fn name(&self) -> &str {
    "emoteonly"
  }
}

impl OnOff for EmoteOnly {}

impl Command for EmoteOnly {}
