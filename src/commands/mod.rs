pub mod ban;
pub mod timeout;
pub mod slow;
pub mod version;
pub mod onoff;

use CommandInfo;
use clap::ArgMatches;

/// A trait for giving something a name.
pub trait HasName {
  /// Gets the name. More meaningful in context.
  fn name(&self) -> &str;
}

/// Defines an entry point into a command struct.
pub trait HasEntryPoint {
  /// The entry point into the command.
  ///
  /// This expects to receive `CommandInfo` from the base command and `ArgMatches` for this specific
  /// subcommand.
  fn entry<'a>(&self, command_info: &CommandInfo, matches: &ArgMatches<'a>);
}

/// A marker trait used to define a command.
pub trait Command: HasName + HasEntryPoint {}
