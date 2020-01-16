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

use std::io;
use std::io::prelude::*;
use std::process::exit;

use hl::highlighters::{get_theme, highlight_line};
use hl::parsers;
use hl::strings;

use getopts::Matches;

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn main() {
    const SYNTAX_DEFAULT: &str = "txt";

    let args: Vec<String> = std::env::args().collect();

    let opts = parsers::parse_options();
    let matches: Matches;
    match parsers::parse_matches(&args, opts) {
        Ok(m) => matches = m,
        Err(e) => {
            println!("Error: {}", e);
            exit(1);
        }
    };

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
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let user_syntax: String;
    match matches.opt_str("syntax") {
        Some(n) => user_syntax = n,
        None => user_syntax = SYNTAX_DEFAULT.to_string(), // use SYNTAX_DEFAULT when not specified
    };

    let mut user_theme = "default"; // default theme
    if matches.opt_present("light") {
        user_theme = "light";
    } else if matches.opt_present("dark") {
        user_theme = "dark";
    }

    let syntax = ss
        .find_syntax_by_token(&user_syntax)
        .unwrap_or_else(|| ss.find_syntax_by_token(&SYNTAX_DEFAULT).unwrap()); // use SYNTAX_DEFAULT if request not found
    let mut hl = HighlightLines::new(syntax, &ts.themes[&get_theme(user_theme)]);

    for line in io::stdin().lock().lines() {
        match line {
            Ok(n) => {
                println!("{}", highlight_line(&n, &mut hl, &ss));
            }
            Err(error) => {
                println!("Error: {}", error);
                exit(1);
            }
        }
    }
}
