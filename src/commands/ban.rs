use {CommandInfo, command, echo};
use clap::ArgMatches;
use commands::{Command, HasEntryPoint, HasName};

type BanFunction = Box<Fn(&str) -> ()>;

pub struct Ban;

impl HasName for Ban {
  fn name(&self) -> &str {
    "ban"
  }
}

impl HasEntryPoint for Ban {
  fn entry<'a>(&self, _: &CommandInfo, matches: &ArgMatches<'a>) {
    let (func, sub) = if let Some(sub) = matches.subcommand_matches("add") {
      (Box::new(add_ban) as BanFunction, sub)
    } else if let Some(sub) = matches.subcommand_matches("remove") {
      (Box::new(remove_ban) as BanFunction, sub)
    } else {
      echo("Unsupported ban operation.");
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
