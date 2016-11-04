use clap::ArgMatches;
use CommandInfo;
use commands::{HasEntryPoint, HasName};
use {echo, command};

pub mod r9kbeta;
pub mod emoteonly;
pub mod subscribers;

/// A marker trait to indicate that this subcommand is an on/off command.
///
/// On/off commands are toggleable, using the name from the `HasName` trait to turn on the command,
/// and using the name plus the word "off" (e.g. "commandoff") to turn off the command.
pub trait OnOff {}

impl<T> HasEntryPoint for T where T: OnOff + HasName {
  fn entry<'a>(&self, _: &CommandInfo, matches: &ArgMatches<'a>) {
    if matches.subcommand_matches("on").is_some() {
      command(self.name(), None);
    } else if matches.subcommand_matches("off").is_some() {
      command(&format!("{}off", self.name()), None);
    } else {
      echo(&format!("Subcommand invalid for {}.", self.name()));
    }
  }
}
