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

// ==================
//
//  themes Module
//
// ==================
use std::error::Error;

use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;

use technicolor::build::syntect::theme::build_themeset_with_names;

pub fn get_theme_set() -> Result<ThemeSet, Box<dyn Error>> {
    let names = ["Ayu-Dark", "Dracula", "Material"];
    build_themeset_with_names(&names)
//    ThemeSet::load_from_folder("assets/themes").unwrap()
}

pub fn get_theme_set_from_binary() -> ThemeSet {
    from_binary(include_bytes!("../../assets/themes.pack"))
}
