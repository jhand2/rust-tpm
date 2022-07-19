// TODO: This interface definition is very C-like. I know the Rust way is to
// use traits, but it ended up a mess of generics and lifetimes. This seemed
// easier. But ultimately we should fix this.
#[derive(Clone, Copy)]
pub struct TpmPlatform {
    pub log: fn(&str),
}

impl Default for TpmPlatform {
    fn default() -> TpmPlatform {
        TpmPlatform { log: default_log }
    }
}

pub fn default_log(_msg: &str) {
    ()
}
