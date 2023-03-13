/*
 * Copyright 2018 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

fn main() {
    lalrpop();
}

fn lalrpop() {
    // cource is first cli argument
    let source = std::env::args().nth(1).unwrap();
    let ruleDir = std::env::args().nth(2).unwrap();
    println!("cargo:rerun-if-changed={}", source);
    lalrpop::Configuration::new()
        // .use_cargo_dir_conventions()
        .set_out_dir(ruleDir)
        .set_in_dir("src")
        .emit_report(true)
        .process_file(source)
        .unwrap();
}
