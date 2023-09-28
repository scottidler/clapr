use crate::arg::Arg;

pub struct Subcommand {
    pub name: String,
    pub description: String,
    pub args: Vec<Arg>,
    // Additional fields for subcommand-specific settings
}

impl Subcommand {
    // Methods for adding arguments, etc.
}
