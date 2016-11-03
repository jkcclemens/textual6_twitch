use clap::ArgMatches;

use {echo, command};

type BanFunction = Box<Fn(&str) -> ()>;

pub fn ban<'a>(matches: &ArgMatches<'a>) {
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

fn add_ban(who: &str) {
  command("ban", Some(&[who]));
}

fn remove_ban(who: &str) {
  command("unban", Some(&[who]));
}
