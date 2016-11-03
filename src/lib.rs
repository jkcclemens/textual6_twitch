extern crate clap;

mod commands;

use clap::ArgMatches;
use commands::ban;
use commands::slow;
use commands::timeout;
use commands::version;
use std::collections::HashMap;

type SubCommandFunction<'a> = Box<Fn(&ArgMatches<'a>) -> ()>;

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
  for (name, func) in build_subcommand_map() {
    if let Some(sub) = info.matches.subcommand_matches(name) {
      func(sub);
      return;
    }
  }
  echo("Not yet implemented.");
}

fn build_subcommand_map<'a>() -> HashMap<&'a str, SubCommandFunction<'a>> {
  let mut map: HashMap<&str, SubCommandFunction<'a>> = HashMap::new();
  map.insert("ban", Box::new(ban::ban));
  map.insert("timeout", Box::new(timeout::timeout));
  map.insert("slow", Box::new(slow::slow));
  map.insert("version", Box::new(version::version));
  map
}
