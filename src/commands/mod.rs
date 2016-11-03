pub mod ban;
pub mod timeout;
pub mod slow;
pub mod version;
pub mod onoff;

use clap::ArgMatches;
use CommandInfo;

pub trait HasName {
  fn name(&self) -> &str;
}

pub trait HasEntryPoint {
  fn entry<'a>(&self, command_info: &CommandInfo, matches: &ArgMatches<'a>);
}

pub trait Command: HasName + HasEntryPoint {}
