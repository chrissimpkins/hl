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

//use syntect::dumps::{dump_to_file, from_binary, from_reader};
use std::process::exit;

use syntect::dumps::{dump_to_file};

use hl::syntaxes::get_syntax_set;
use hl::themes::get_theme_set;

const THEMES_BIN_PATH: &str = "assets/themes.bin";
const SYNTAXES_BIN_PATH: &str = "assets/syntaxes.bin";

pub fn main() {
    build_themes();
    build_syntaxes();
}

fn build_themes() {
    let ts = get_theme_set();
    match dump_to_file(&ts, &THEMES_BIN_PATH) {
        Ok(_) => { println!("Built theme set at {}", THEMES_BIN_PATH); },
        Err(e) => { eprintln!("Failed to build theme set: Error: {}", e); exit(1)},
    }
}

fn build_syntaxes() {
    let ss = get_syntax_set();
    match dump_to_file(&ss, SYNTAXES_BIN_PATH) {
        Ok(_) => { println!("Built syntax set at {}", SYNTAXES_BIN_PATH); },
        Err(e) => { eprintln!("Failed to build syntax set: Error: {}", e); exit(1)},
    }
}
