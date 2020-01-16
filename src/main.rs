use getopts::Options;
//use std::io;
//use std::io::prelude::*;
use std::process::exit;

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
}
