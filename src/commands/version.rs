use clap::ArgMatches;

use echo;

pub fn version<'a>(_: &ArgMatches<'a>) {
  echo("textual6_twitch 0.1.0"); // TODO: use crate info
}
