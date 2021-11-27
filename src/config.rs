use syn::parse::{Parse, Result, ParseStream};

pub struct Config {
    allow_self_sized: bool,
    name: Option<String>,
}

impl Config {
    pub fn allow_self_sized(&self) -> bool {
        self.allow_self_sized
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }
}

impl Parse for Config {
    fn parse(_: ParseStream) -> Result<Self> {
        Ok(
            Config {
                allow_self_sized: false,
                name: None
            }
        )
    }
}
