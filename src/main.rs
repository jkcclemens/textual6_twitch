extern crate textual6_twitch;
extern crate clap;

use textual6_twitch::{CommandInfo, entry};
use clap::{Arg, App, AppSettings, SubCommand};
use std::env::args;

fn main() {
  // Collect all command-line args into a vec
  let args: Vec<String> = args().collect();
  // Ensure correct args length
  if args.len() < 3 {
    println!("/echo Incorrect number of arguments passed. Maybe Textual was updated?");
    return;
  }
  // The second is always the channel (or an empty string for no channel)
  let channel = match args[1].as_str() {
    "" => None,
    x => Some(x.to_string())
  };
  // The rest are the arguments supplied to the command, with an empty string for no args
  let mut real_args: Vec<String> = args[2..].to_vec();
  // Remove the empty string if there are no args
  if real_args.len() == 1 && real_args[0].is_empty() {
    real_args.remove(0);
  }
  // Do CLI matching with clap
  let matches = App::new("textual6_twitch")
    .version("0.1.0") // TODO: use crate info
    .about("Twitch IRC commands for Textual 6")
    .bin_name("/twitch")
    .global_setting(AppSettings::VersionlessSubcommands)
    .global_setting(AppSettings::NoBinaryName)
    .global_setting(AppSettings::ColorNever)
    .global_setting(AppSettings::DisableVersion)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("version"))
    .subcommand(SubCommand::with_name("ban")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("add")
        .arg(Arg::with_name("who")
          .required(true)
          .takes_value(true)
          .value_name("name")
          .help("The target of the ban.")))
      .subcommand(SubCommand::with_name("remove")
        .arg(Arg::with_name("who")
          .required(true))))
    .subcommand(SubCommand::with_name("timeout")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("add")
        .arg(Arg::with_name("who")
          .required(true)
          .takes_value(true)
          .value_name("name")
          .help("The target of the timeout."))
        .arg(Arg::with_name("duration")
          .required(false)))
      .subcommand(SubCommand::with_name("remove")
        .arg(Arg::with_name("who")
          .required(true))))
    .subcommand(SubCommand::with_name("slow")
      .about("Slow mode configuration. Takes a duration in seconds or 0 to disable. The duration is how long users must wait between messages.")
      .arg(Arg::with_name("duration")
        .help("The duration of slow mode. Use 0 to disable.")
        .required(false)
        .takes_value(true)
        .value_name("seconds")))
    .get_matches_from_safe(real_args);
  // Preface each error line with /echo if there was an error
  let matches = match matches {
    Ok(m) => m,
    Err(e) => {
      let e: String = e.to_string()
        .split('\n')
        .map(|line| "/echo ".to_string() + line + "\n")
        .collect();
      println!("{}", e);
      println!("/echo --------------");
      return;
    }
  };
  // Create the CommandInfo struct
  let ci = CommandInfo::new(channel, matches);
  // Call the entry point
  entry(ci);
}
