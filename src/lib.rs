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

pub mod highlighters;
pub mod parsers;
pub mod strings;

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


#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::settings::{EXECUTABLE, LICENSE, VERSION};

    #[test]
    fn test_executable_name() {
        assert_eq!(EXECUTABLE, "hl");
    }

    #[test]
    fn test_executable_version() {
        let re = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        assert!(re.is_match(VERSION));
    }

    #[test]
    fn test_license() {
        assert_eq!(LICENSE, "Apache License, v2.0");
    }
}