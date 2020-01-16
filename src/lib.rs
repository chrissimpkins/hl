// Copyright 2019 Christopher Simpkins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ==================
//
//  Parsers Module
//
// ==================

pub mod parsers {
    use getopts::{Matches, Options};

    pub fn parse_options() -> Options {
        let mut opts = Options::new();
        opts.optflag("l", "light", "Light highlight mode");
        opts.optflag("d", "dark", "Dark highlight mode");
        opts.optopt("s", "syntax", "Source syntax format", "SYNTAX");
        opts.optflag("h", "help", "Print this help menu");
        opts.optflag("v", "version", "Print version number");

        opts
    }

    pub fn parse_matches(args: &[String], opts: Options) -> Matches {
        // parse command line arguments
        match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        }
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
    pub const HELP: &str = r#"Pipe the standard output stream from an executable to the hl executable. Use options to define the piped source format and modify the default syntax highlighting color scheme.
    "#;
} // END settings module

// ==================
//
//  Strings Module
//
// ==================
pub mod strings {
    pub use crate::parsers;
    pub use crate::settings;

    pub fn get_help() -> String {
        let opts = parsers::parse_options();

        let usage_pre = format!("Usage: {} [options]", settings::EXECUTABLE);
        let usage = opts.usage(&usage_pre);

        format!(
            r#":::::::::::::::::::::::::::::::::::::
 {} v{}
 {}
 {}
 {}

 {}
:::::::::::::::::::::::::::::::::::::


{}

{}
"#,
            settings::EXECUTABLE,
            settings::VERSION,
            settings::DESCRIPTION,
            settings::COPYRIGHT,
            settings::LICENSE,
            settings::SOURCE_REPOSITORY,
            usage,
            settings::HELP,
        ) // END format! of returned String
    }

    pub fn get_version() -> String {
        format!("{} v{}", settings::EXECUTABLE, settings::VERSION)
    }
} // END strings module
