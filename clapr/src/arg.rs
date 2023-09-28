pub enum ArgType {
    Flag,
    Option,
    Positional,
    // Add more types as needed
}

pub struct Arg {
    pub name: String,
    pub description: String,
    pub arg_type: ArgType,
    // Additional fields for default values, validators, etc.
}

impl Arg {
    // Methods for setting default values, validators, etc.
}
