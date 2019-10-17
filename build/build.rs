// Copyright 2019 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod find_normal;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[cfg(feature = "vendored")]
pub fn vendored() {
    println!("cargo:rerun-if-changed=build.rs");
    let artifacts = mbedtls_src::Build::new().build();
    artifacts.print_cargo_metadata();

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out_dir.join("include"))
        .unwrap()
        .write_all(artifacts.include_dir().to_str().unwrap().as_bytes())
        .unwrap();
    File::create(out_dir.join("lib"))
        .unwrap()
        .write_all(artifacts.lib_dir().to_str().unwrap().as_bytes())
        .unwrap();
    File::create(out_dir.join("target"))
        .unwrap()
        .write_all(env::var("TARGET").unwrap().as_bytes())
        .unwrap();
    File::create(out_dir.join("mbedtls-src-version"))
        .unwrap()
        .write_all(mbedtls_src::version().as_bytes())
        .unwrap();
}


fn main() {
    //let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR undefined");
    //let out_dir = std::path::PathBuf::from(out_dir);

    #[cfg(feature = "vendored")]
    vendored();

    #[cfg(not(feature = "vendored"))]
    find_normal::find_normal();
}
