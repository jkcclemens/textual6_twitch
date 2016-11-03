use clap::ArgMatches;

use commands::{Command, HasEntryPoint, HasName};
use {CommandInfo, echo, command};

pub struct Timeout;

impl Command for Timeout {}

impl HasName for Timeout {
  fn name(&self) -> &str {
    "timeout"
  }
}

impl HasEntryPoint for Timeout {
  fn entry<'a>(&self, _: &CommandInfo, matches: &ArgMatches<'a>) {
    if let Some(sub) = matches.subcommand_matches("add") {
      add_timeout(sub);
    } else if let Some(sub) = matches.subcommand_matches("remove") {
      remove_timeout(sub);
    } else {
      echo("Unsupported timeout operation.");
    }
  }
}

fn add_timeout<'a>(matches: &ArgMatches<'a>) {
  let who = match matches.value_of("who") {
    Some(w) => w,
    None => {
      echo("No target for timeout.");
      return;
    }
  };
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

fn remove_timeout<'a>(matches: &ArgMatches<'a>) {
  let who = match matches.value_of("who") {
    Some(w) => w,
    None => {
      echo("No target for timeout.");
      return;
    }
  };
  command("untimeout", Some(&[who]));
}
