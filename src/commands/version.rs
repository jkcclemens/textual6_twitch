use clap::ArgMatches;

use {echo, CommandInfo};
use commands::Command;

pub struct Version;

impl Command for Version {
  fn name(&self) -> &str {
    "version"
  }

  fn entry<'a>(&self, _: &CommandInfo, _: &ArgMatches<'a>) {
    echo("textual6_twitch 0.1.0"); // TODO: use crate info
  }
}
