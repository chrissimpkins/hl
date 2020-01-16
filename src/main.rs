use getopts::Options;
use std::io;
use std::io::prelude::*;
use std::process::exit;

use hl::settings::{COPYRIGHT, DESCRIPTION, EXECUTABLE, HELP, LICENSE, SOURCE_REPOSITORY, VERSION};

use hl::parse_options;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();

    let matches = parse_options(&args);

    // ==================
    //
    //  Help
    //
    // ==================
    if matches.opt_present("help") {
        println!("===================================");
        println!("{} v{}", EXECUTABLE, VERSION);
        println!("{}", DESCRIPTION);
        println!("{}", COPYRIGHT);
        println!("{}", LICENSE);
        println!("{}", SOURCE_REPOSITORY);
        println!("===================================");
        println!();
        let help_brief = format!("Usage: {} [options]", EXECUTABLE);
        print!("{}", opts.usage(&help_brief));
        println!();
        println!("{}", HELP);

        exit(0);
    }

    // ==================
    //
    //  Version
    //
    // ==================
    if matches.opt_present("version") {
        println!("{} v{}", EXECUTABLE, VERSION);
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
