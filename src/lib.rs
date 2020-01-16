




// ==================
//
//  Parsers Module
//
// ==================

pub mod parsers {
    use getopts::{Matches, Options};

    pub fn parse_options(args: &[String]) -> Matches {
        let mut opts = Options::new();
        opts.optflag("l", "light", "Light mode");
        opts.optflag("d", "dark", "Dark mode");
        opts.optopt("s", "syntax", "Source syntax format", "SYNTAX");
        opts.optflag("h", "help", "Print this help menu");
        opts.optflag("v", "version", "Print version number");

        // parse command line arguments
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        return matches;
    }

} // END parsers module


// ==================
//
//  Settings Module
//
// ==================
pub mod settings {
    pub const EXECUTABLE: &str = env!("CARGO_PKG_NAME");
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    pub const COPYRIGHT: &str = "Copyright 2019 Christopher Simpkins";
    pub const LICENSE: &str = "Apache License, v2.0";
    pub const SOURCE_REPOSITORY: &str = "https://github.com/chrissimpkins/hl";
    pub const HELP: &str = r#"Pipe the standard output stream from an executable to the hl executable and use options to define the piped source format and syntax highlighting color scheme.
    "#;
} // END settings module
