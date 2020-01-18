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
//  strings Module
//
// ==================

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


#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_version_string() {
        let re = Regex::new(r"^hl\sv\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        assert!(re.is_match(&crate::strings::get_version()))
    }

}