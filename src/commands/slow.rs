use clap::ArgMatches;

use commands::{Command, HasEntryPoint, HasName};
use {CommandInfo, echo, command};

pub struct Slow;

impl Command for Slow {}

impl HasName for Slow {
  fn name(&self) -> &str {
    "slow"
  }
}

impl HasEntryPoint for Slow {
  fn entry<'a>(&self, _: &CommandInfo, matches: &ArgMatches<'a>) {
    if let Some(duration) = matches.value_of("duration") {
      let duration = match duration.parse::<usize>() {
        Ok(d) => d,
        Err(e) => {
          echo(&format!("Could not parse duration: {}", e));
          return;
        }
      };
      if duration == 0 {
        command("slowoff", None);
      } else {
        command("slow", Some(&[&duration.to_string()]));
      }
      return;
    };
    command("slow", None);
  }
}
