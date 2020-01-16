use getopts::Options;
use std::io;
use std::io::prelude::*;
use std::process::exit;

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;

fn main() {
    const EXECUTABLE: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    const COPYRIGHT: &str = "Copyright 2019 Christopher Simpkins";
    const LICENSE: &str = "Apache License, v2.0";
    const SOURCE_REPOSITORY: &str = "https://github.com/chrissimpkins/hl";
    const HELP: &str = r#"Pipe the standard output stream from an executable to the hl executable and use options to define the piped source format and syntax highlighting color scheme.
    "#;

    let args: Vec<String> = std::env::args().collect();

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

    let syntax = ps.find_syntax_by_token(&user_syntax)
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
