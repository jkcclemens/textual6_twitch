pub mod ban;
pub mod timeout;
pub mod slow;
pub mod version;

use clap::ArgMatches;
use CommandInfo;

pub trait Command {
  fn name(&self) -> &str;

  fn entry<'a>(&self, command_info: &CommandInfo, matches: &ArgMatches<'a>);
}
