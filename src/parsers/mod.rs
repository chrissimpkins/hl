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
//  parsers Module
//
// ==================

use getopts::{Matches, Options};
use std::io;

pub fn parse_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("l", "light", "Light highlight mode");
    opts.optflag("d", "dark", "Dark highlight mode");
    opts.optopt("s", "syntax", "Source syntax format", "SYNTAX");
    // todo: implement custom theme support
//    opts.optopt("t", "theme", "Theme name", "THEME");
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("v", "version", "Print version number");

    opts
}

pub fn parse_matches(args: &[String], opts: Options) -> Result<Matches, io::Error> {
    // parse command line arguments
    match opts.parse(&args[1..]) {
        Ok(m) => Ok(m),
        Err(f) => Err(io::Error::new(io::ErrorKind::Other, f.to_string())),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_matches_valid_empty() {
        let opts = crate::parsers::parse_options();
        let argv = ["hl".to_string()];
        let matches = crate::parsers::parse_matches(&argv, opts);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_parse_matches_valid_one_valid_option() {
        let opts = crate::parsers::parse_options();
        let argv = ["hl".to_string(), "-l".to_string()];
        let matches = crate::parsers::parse_matches(&argv, opts);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_parse_matches_valid_two_valid_options() {
        let opts = crate::parsers::parse_options();
        let argv = [
            "hl".to_string(),
            "-l".to_string(),
            "-s".to_string(),
            "txt".to_string(),
        ];
        let matches = crate::parsers::parse_matches(&argv, opts);
        assert!(matches.is_ok());
    }

    #[test]
    fn test_parse_matches_invalid_unsupported_option() {
        let opts = crate::parsers::parse_options();
        let argv = ["hl".to_string(), "--bogus".to_string()];
        let matches = crate::parsers::parse_matches(&argv, opts);
        assert!(matches.is_err());
    }

    #[test]
    fn test_parse_matches_invalid_missing_argument() {
        let opts = crate::parsers::parse_options();
        let argv = ["hl".to_string(), "--syntax".to_string()];
        let matches = crate::parsers::parse_matches(&argv, opts);
        assert!(matches.is_err());
    }
}
