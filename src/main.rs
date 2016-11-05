extern crate textual6_twitch;
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env::args;
use textual6_twitch::{CommandInfo, entry};

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
    .subcommand(SubCommand::with_name("version")
      .about("Gets the version of textual6_twitch."))
    .subcommand(SubCommand::with_name("ban")
      .alias("bans")
      .about("Manages channel bans.")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("add")
        .about("Adds a ban to the channel.")
        .arg(Arg::with_name("who")
          .required(true)
          .takes_value(true)
          .value_name("name")
          .help("The target of the ban to add.")))
      .subcommand(SubCommand::with_name("remove")
        .about("Removes a ban from the channel.")
        .arg(Arg::with_name("who")
          .required(true)
          .help("The target of the ban to remove."))))
    .subcommand(SubCommand::with_name("timeout")
      .alias("timeouts")
      .about("Manages channel timeouts.")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("add")
        .about("Adds a timeout to the channel.")
        .arg(Arg::with_name("who")
          .required(true)
          .value_name("name")
          .help("The target of the timeout to add."))
        .arg(Arg::with_name("duration")
          .required(false)
          .value_name("seconds")
          .help("The number of seconds to time the user out for.")))
      .subcommand(SubCommand::with_name("remove")
        .arg(Arg::with_name("who")
          .required(true)
          .help("The target of the timeout to remove."))))
    .subcommand(SubCommand::with_name("slow")
      .about("Slow mode configuration.")
      .arg(Arg::with_name("duration")
        .help("The duration of slow mode, which limits users to one message every so many seconds. Use 0 to disable.")
        .required(false)
        .value_name("seconds")))
    .subcommand(SubCommand::with_name("r9kbeta")
      .alias("r9k")
      .about("R9K mode configuration.")
      .before_help("R9K mode ensures users can only post unique messages to the channel.")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("on"))
      .subcommand(SubCommand::with_name("off")))
    .subcommand(SubCommand::with_name("emoteonly")
      .aliases(&["emotesonly", "emote", "emotes"])
      .about("Emote-only mode configuration.")
      .before_help("Emote-only mode ensures users can only post messages that are 100% emotes.")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("on"))
      .subcommand(SubCommand::with_name("off")))
    .subcommand(SubCommand::with_name("subscribers")
      .aliases(&["subs", "subsonly", "subscribersonly"])
      .about("Subscribers mode configuration.")
      .before_help("Subscribers mode ensures that only subscribers, mods, admins, and Twitch staff can post messages.")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("on"))
      .subcommand(SubCommand::with_name("off")))
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
