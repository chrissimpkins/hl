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

//use getopts::Options;
use std::io;
use std::io::prelude::*;
use std::process::exit;

use hl::parsers;
use hl::strings;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let opts = parsers::parse_options();
    let matches = parsers::parse_matches(&args, opts);

    // ==================
    //
    //  Help
    //
    // ==================
    if matches.opt_present("help") {
        print!("{}", strings::get_help());

        exit(0);
    }

    // ==================
    //
    //  Version
    //
    // ==================
    if matches.opt_present("version") {
        println!("{}", strings::get_version());
        exit(0);
    }

    // ====================
    //
    //  Syntax highlighting
    //  execution
    //
    // ====================
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let user_syntax: String;
    match matches.opt_str("syntax") {
        Some(n) => user_syntax = n,
        None => user_syntax = "txt".to_string(),
    };

    let syntax = ps
        .find_syntax_by_token(&user_syntax)
        .unwrap_or_else(|| ps.find_syntax_by_token("txt").unwrap());

    // InspiredGitHub
    // Solarized (dark)
    // Solarized (light)
    // base16-eighties.dark
    // base16-mocha.dark
    // base16-ocean.dark
    // base16-ocean.light
    let mut user_theme = "base16-eighties.dark"; // default theme
    if matches.opt_present("light") {
        user_theme = "Solarized (light)";
    } else if matches.opt_present("dark") {
        user_theme = "base16-ocean.dark";
    }

    let mut h = HighlightLines::new(syntax, &ts.themes[user_theme]);

    for line in io::stdin().lock().lines() {
        match line {
            Ok(n) => {
                let ranges: Vec<(Style, &str)> = h.highlight(&n, &ps);
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                println!("{}", escaped);
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
