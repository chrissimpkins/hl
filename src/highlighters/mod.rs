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

// ====================
//
//  highlighters Module
//
// ====================

use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

// Theme options built into syntect library:
// InspiredGitHub
// Solarized (dark)
// Solarized (light)
// base16-eighties.dark
// base16-mocha.dark
// base16-ocean.dark
// base16-ocean.light
const THEME_DEFAULT: &str = "base16-eighties.dark";
const THEME_DARK: &str = "base16-ocean.dark";
const THEME_LIGHT: &str = "base16-ocean.light";

pub fn get_theme(user_request: &str) -> String {
    if user_request == "light" {
        THEME_LIGHT.to_string()
    } else if user_request == "dark" {
        THEME_DARK.to_string()
    } else {
        THEME_DEFAULT.to_string()
    }
}

pub fn highlight_line(line: &str, hl: &mut HighlightLines, ss: &SyntaxSet) -> String {
    let ranges: Vec<(Style, &str)> = hl.highlight(&line, ss);

    as_24_bit_terminal_escaped(&ranges[..], true)
}