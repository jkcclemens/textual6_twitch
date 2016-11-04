use clap::ArgMatches;

use commands::{Command, HasEntryPoint, HasName};
use {CommandInfo, echo, command};

type TimeoutFunction = Box<Fn(&str, &ArgMatches) -> ()>;

pub struct Timeout;

impl Command for Timeout {}

impl HasName for Timeout {
  fn name(&self) -> &str {
    "timeout"
  }
}

impl HasEntryPoint for Timeout {
  fn entry<'a>(&self, _: &CommandInfo, matches: &ArgMatches<'a>) {
    let (func, sub) = if let Some(sub) = matches.subcommand_matches("add") {
      (Box::new(add_timeout) as TimeoutFunction, sub)
    } else if let Some(sub) = matches.subcommand_matches("remove") {
      (Box::new(remove_timeout) as TimeoutFunction, sub)
    } else {
      echo("Unsupported timeout operation.");
      return;
    };
    let who = match sub.value_of("who") {
      Some(w) => w,
      None => {
        echo("No target for timeout.");
        return;
      }
    };
    func(who, sub);
  }
}

fn add_timeout(who: &str, matches: &ArgMatches) {
  match matches.value_of("duration") {
    Some(d) => {
      match d.parse::<usize>() {
        Ok(dur) => {
          if dur > 1209600 {
            echo("Duration may not exceed 1209600.");
            return;
          }
        },
        Err(e) => {
          echo(&format!("Error parsing duration: {}", e));
          return;
        }
      }
      command("timeout", Some(&[who, d]));
    },
    None => command("timeout", Some(&[who]))
  }
}

fn remove_timeout(who: &str, _: &ArgMatches) {
  command("untimeout", Some(&[who]));
}
