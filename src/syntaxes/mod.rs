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
//  syntaxes Module
//
// ====================
use std::error::Error;

use syntect::dumps::from_binary;
use syntect::parsing::SyntaxSet;

use technicolor::build::syntect::syntax::build_full_syntaxset_with_newlines;

pub fn get_syntax_set() -> SyntaxSet {
//    let ss = SyntaxSet::load_defaults_newlines();
//    let mut ssb = ss.into_builder();
//    ssb.add_from_folder("assets/syntaxes", true).unwrap();
//    ssb.build()
    build_full_syntaxset_with_newlines()
}

pub fn get_syntax_set_from_binary() -> SyntaxSet {
    from_binary(include_bytes!("../../assets/syntaxes-nl.pack"))
}
