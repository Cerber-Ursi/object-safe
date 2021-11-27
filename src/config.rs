use syn::{
    parse::{Parse, ParseStream},
    Result, Token,
};

#[derive(Default, Debug)]
pub struct Config {
    allow_self_sized: bool,
    name: Option<String>,
}

struct RawConfigAllowFirst {
    _allow_self_sized: AllowSelfSized,
    name: Option<Name>,
}
struct RawConfigNameFirst {
    name: Name,
    allow_self_sized: Option<AllowSelfSized>,
}

struct AllowSelfSized;
struct Name(String);

enum RawConfig {
    AllowFirst(RawConfigAllowFirst),
    NameFirst(RawConfigNameFirst),
}

impl Config {
    pub fn allow_self_sized(&self) -> bool {
        self.allow_self_sized
    }
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }
}

impl From<RawConfig> for Config {
    fn from(input: RawConfig) -> Config {
        let mut ret: Config = Default::default();
        match input {
            RawConfig::AllowFirst(cfg) => {
                ret.allow_self_sized = true;
                ret.name = cfg.name.map(|name| name.0);
            }
            RawConfig::NameFirst(cfg) => {
                ret.name = Some(cfg.name.0);
                ret.allow_self_sized = cfg.allow_self_sized.is_some();
            }
        }
        ret
    }
}

impl Parse for AllowSelfSized {
    fn parse(stream: ParseStream) -> Result<Self> {
        let ident: syn::Ident = stream.parse()?;
        if ident.to_string().as_str() == "allow_sized" {
            Ok(AllowSelfSized)
        } else {
            Err(stream.error("Unexpected token"))
        }
    }
}
impl Parse for Name {
    fn parse(stream: ParseStream) -> Result<Self> {
        let ident: syn::Ident = stream.parse()?;
        if ident.to_string().as_str() == "name" {
            let _: Token![=] = stream.parse()?;
            let name: syn::Ident = stream.parse()?;
            Ok(Name(name.to_string()))
        } else {
            Err(stream.error("Unexpected token"))
        }
    }
}
impl Parse for RawConfigAllowFirst {
    fn parse(stream: ParseStream) -> Result<Self> {
        Ok(RawConfigAllowFirst {
            _allow_self_sized: stream.parse()?,
            name: if stream.is_empty() {
                None
            } else {
                let _: Token![,] = stream.parse()?;
                Some(stream.parse()?)
            },
        })
    }
}
impl Parse for RawConfigNameFirst {
    fn parse(stream: ParseStream) -> Result<Self> {
        Ok(RawConfigNameFirst {
            name: stream.parse()?,
            allow_self_sized: if stream.is_empty() {
                None
            } else {
                let _: Token![,] = stream.parse()?;
                Some(stream.parse()?)
            },
        })
    }
}
impl Parse for RawConfig {
    fn parse(stream: ParseStream) -> Result<Self> {
        let streaf = stream.fork();
        match (
            streaf.parse::<RawConfigAllowFirst>(),
            stream.parse::<RawConfigNameFirst>(),
        ) {
            (Ok(cfg), _) => Ok(RawConfig::AllowFirst(cfg)),
            (Err(_), Ok(cfg)) => Ok(RawConfig::NameFirst(cfg)),
            (Err(mut err1), Err(err2)) => {
                err1.combine(err2);
                Err(err1)
            }
        }
    }
}

impl Parse for Config {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            Ok(Default::default())
        } else {
            let raw: RawConfig = stream.parse()?;
            Ok(raw.into())
        }
    }
}
