use darling::FromMeta;

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
pub struct Config {
    #[darling(rename = "allow_sized")]
    allow_self_sized: darling::util::Flag,
    name: Option<darling::util::IdentString>,
}

impl Config {
    pub fn allow_self_sized(&self) -> bool {
        self.allow_self_sized.is_some()
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(darling::util::IdentString::as_str)
    }
}


