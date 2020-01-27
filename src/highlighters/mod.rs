// Copyright 2020 Christopher Simpkins
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
use std::fmt::Write;

use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
//use syntect::util::as_24_bit_terminal_escaped;

// Theme options built into syntect library:
// InspiredGitHub
// Solarized (dark)
// Solarized (light)
// base16-eighties.dark
// base16-mocha.dark
// base16-ocean.dark
// base16-ocean.light
const THEME_DARK: &str = "Dracula";
const THEME_DEFAULT: &str = "PreDawn";
const THEME_LIGHT: &str = "Ayu-Light";

pub fn get_theme(user_request: &str) -> String {
    match user_request {
        "light" => THEME_LIGHT.to_string(),
        "dark" => THEME_DARK.to_string(),
        _ => THEME_DEFAULT.to_string(),
    }
}

pub fn highlight_line(line: &str, hl: &mut HighlightLines, ss: &SyntaxSet) -> String {
    let mut pre_string = line.to_string();
    pre_string.push_str("\n");
    let ranges: Vec<(Style, &str)> = hl.highlight(&pre_string, ss);

    as_24_bit_terminal_escaped(&ranges[..], true)
}

pub fn as_24_bit_terminal_escaped(v: &[(Style, &str)], bg: bool) -> String {
    let mut s: String = String::new();
    for &(ref style, text) in v.iter() {
        if bg {
            write!(
                s,
                "\x1b[38;2;{};{};{};48;2;{};{};{}m{}",
                style.foreground.r,
                style.foreground.g,
                style.foreground.b,
                style.background.r,
                style.background.g,
                style.background.b,
                text
            )
            .unwrap();
        } else {
            write!(
                s,
                "\x1b[38;2;{};{};{}m{}",
                style.foreground.r, style.foreground.g, style.foreground.b, text
            )
            .unwrap();
        }
    }
    //    s.push_str("\x1b[0m");
    s
}

#[cfg(test)]
mod tests {

    const TESTLINE: &str = r#"      <coord axis="opsz" value="11.0"/>"#;
    const EXPECTED_LINE_BEGIN: &str = "\u{1b}[38;2;21";
    #[test]
    fn test_get_theme_default() {
        assert_eq!(
            crate::highlighters::get_theme("default"),
            crate::highlighters::THEME_DEFAULT
        );
    }

    #[test]
    fn test_get_theme_dark() {
        assert_eq!(
            crate::highlighters::get_theme("dark"),
            crate::highlighters::THEME_DARK
        );
    }

    #[test]
    fn test_get_theme_light() {
        assert_eq!(
            crate::highlighters::get_theme("light"),
            crate::highlighters::THEME_LIGHT
        );
    }

    #[test]
    fn test_highlight_line_xml() {
        let ss = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let ts = syntect::highlighting::ThemeSet::load_defaults();
        let syntax = ss
            .find_syntax_by_token("xml")
            .unwrap_or_else(|| ss.find_syntax_by_token("txt").unwrap());
        let mut hl = syntect::easy::HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);
        let escaped_string = crate::highlighters::highlight_line(TESTLINE, &mut hl, &ss);
        assert_eq!(escaped_string.get(0..9), Some(EXPECTED_LINE_BEGIN));
    }
}
