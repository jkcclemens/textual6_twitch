extern crate clap;

mod commands;

use clap::ArgMatches;
use commands::{Command, ban, slow, timeout, version, onoff};

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

/// The entry point for the program's functionality
pub fn entry(info: CommandInfo) {
  // Loop through each subcommand
  for command in build_subcommand_map() {
    // If the subcommand is present in the arg matches
    if let Some(sub) = info.matches.subcommand_matches(command.name()) {
      // Call the subcommand's entry point
      command.entry(&info, sub);
      // And end the progrma
      return;
    }
  }
  // Couldn't find a matching subcommand!
  echo("Not yet implemented.");
}

fn build_subcommand_map<'a>() -> Vec<BoxedCommand> {
  let mut commands: Vec<BoxedCommand> = Vec::new();
  commands.push(Box::new(ban::Ban {}));
  commands.push(Box::new(timeout::Timeout {}));
  commands.push(Box::new(slow::Slow {}));
  commands.push(Box::new(version::Version {}));
  commands.push(Box::new(onoff::r9kbeta::R9KBeta {}));
  commands.push(Box::new(onoff::subscribers::Subscribers {}));
  commands.push(Box::new(onoff::emoteonly::EmoteOnly {}));
  commands
}
