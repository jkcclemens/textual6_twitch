use clap::ArgMatches;
use CommandInfo;
use commands::{HasEntryPoint, HasName};
use {echo, command};

pub mod r9kbeta;
pub mod emoteonly;
pub mod subscribers;

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
