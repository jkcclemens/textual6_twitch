use clap::ArgMatches;

use commands::{Command, HasEntryPoint, HasName};
use {CommandInfo, echo, command};

type BanFunction = Box<Fn(&str) -> ()>;

pub struct Ban;

impl HasName for Ban {
  fn name(&self) -> &str {
    "ban"
  }
}

impl HasEntryPoint for Ban {
  fn entry<'a>(&self, _: &CommandInfo, matches: &ArgMatches<'a>) {
    let func: BanFunction;
    let sub = if let Some(sub) = matches.subcommand_matches("add") {
      func = Box::new(add_ban);
      sub
    } else if let Some(sub) = matches.subcommand_matches("remove") {
      func = Box::new(remove_ban);
      sub
    } else {
      echo("Unsupported timeout operation.");
      return;
    };
    let who = match sub.value_of("who") {
      Some(w) => w,
      None => {
        echo("No target for ban.");
        return;
      }
    };
    func(who);
  }
}

impl Command for Ban {}

fn add_ban(who: &str) {
  command("ban", Some(&[who]));
}

fn remove_ban(who: &str) {
  command("unban", Some(&[who]));
}
