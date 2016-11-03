extern crate clap;

mod commands;

use clap::ArgMatches;
use commands::{Command, ban, slow, timeout, version};

type BoxedCommand = Box<Command>;

#[derive(Debug)]
pub struct CommandInfo<'a> {
  pub channel: Option<String>,
  pub matches: ArgMatches<'a>
}

impl<'a> CommandInfo<'a> {
  pub fn new(channel: Option<String>, matches: ArgMatches<'a>) -> Self {
    CommandInfo {
      channel: channel,
      matches: matches
    }
  }
}

pub fn echo(string: &str) {
  println!("/echo {}", string);
}

pub fn command(name: &str, args: Option<&[&str]>) {
  if let Some(args) = args {
    println!(".{} {}", name, args.join(" "));
  } else {
    println!(".{}", name);
  }
}

pub fn entry(info: CommandInfo) {
  for command in build_subcommand_map() {
    if let Some(sub) = info.matches.subcommand_matches(command.name()) {
      command.entry(&info, sub);
      return;
    }
  }
  echo("Not yet implemented.");
}

fn build_subcommand_map<'a>() -> Vec<BoxedCommand> {
  let mut commands: Vec<BoxedCommand> = Vec::new();
  commands.push(Box::new(ban::Ban {}));
  commands.push(Box::new(timeout::Timeout {}));
  commands.push(Box::new(slow::Slow {}));
  commands.push(Box::new(version::Version {}));
  // TODO: r9kbeta & r9kbetaoff
  // TODO: subscribers & subscribersoff
  // TODO: emoteonly & emoteonlyoff
  commands
}
