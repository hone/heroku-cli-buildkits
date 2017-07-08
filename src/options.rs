extern crate docopt;

#[derive(Deserialize, Debug)]
pub struct Options {
    pub cmd_init: bool,
    pub cmd_register: bool,
    pub cmd_publish: bool,
    pub cmd_rollback: bool,
    pub arg_name: String,
    pub arg_repo: String,
    pub arg_treeish: String,
    pub arg_revision: String,
}

impl Options {
    pub fn new<I: IntoIterator<Item = S>, S: AsRef<str>>(argv: I) -> Options {
        docopt::Docopt::new(self::vars::USAGE)
            .and_then(|d| d.argv(argv.into_iter()).deserialize())
            .unwrap_or_else(|e| e.exit())
    }
}

pub mod vars {
    pub const USAGE: &'static str = "
Heroku Buildpacks CLI

USAGE:
  buildpacks init <name>
  buildpacks register <repo>
  buildpacks publish [<name>] [<treeish>]
  buildpacks rollback <name> <revision>
";

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        let name = "foo";
        let args = vec!["buildpacks", "init", name];
        let options = Options::new(args);

        assert!(options.cmd_init);
        assert_eq!(name, options.arg_name);
    }
}
