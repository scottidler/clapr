use crate::subcommand::Subcommand;

use crate::arg::Arg;

pub struct Command {
    pub name: String,
    pub description: String,
    pub args: Vec<Arg>,
    pub subcommands: Vec<Subcommand>,
    // Additional fields for command-specific settings
}

impl Command {
    // Methods for adding arguments, subcommands, etc.
}
