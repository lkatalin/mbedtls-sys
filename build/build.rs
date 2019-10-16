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
use std::path::{Path, PathBuf};
use std::ffi::OsString;

pub fn get_mbedtls(_target: &str) -> (PathBuf, PathBuf) {
    let artifacts = mbedtls_src::Build::new().build();
    println!("cargo:vendored=1");
    println!("cargo:root={}", artifacts.lib_dir().parent().unwrap().display());

    (
        artifacts.lib_dir().to_path_buf(),
        artifacts.include_dir().to_path_buf(),
    )
}    

fn env_inner(name: &str) -> Option<OsString> {
    let var = env::var_os(name);
    println!("cargo:rerun-if-env-changed={}", name);

    match var {
        Some(ref v) => println!("{} = {}", name, v.to_string_lossy()),
        None => println!("{} unset", name),
    }

    var
}

fn env(name: &str) -> Option<OsString> {
    let prefix = env::var("TARGET").unwrap().to_uppercase().replace("-", "_");
    let prefixed = format!("{}_{}", prefix, name);
    env_inner(&prefixed).or_else(|| env_inner(name))
}

fn find_vendored() {
    let target = env::var("TARGET").unwrap();

    let (lib_dir, include_dir) = get_mbedtls(&target);

    if !Path::new(&lib_dir).exists() {
        panic!(
            "OpenSSL library directory does not exist: {}",
            lib_dir.to_string_lossy()
        );
    }
    if !Path::new(&include_dir).exists() {
        panic!(
            "OpenSSL include directory does not exist: {}",
            include_dir.to_string_lossy()
        );
    }

    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_string_lossy()
    );

    println!("cargo:include={}", include_dir.to_string_lossy());

    let libs_env = env("MBEDTLS_LIBS");
    let libs : Vec<&str> = match libs_env.as_ref().and_then(|s| s.to_str()) {
        Some(ref v) => v.split(":").collect(),
        None => panic!("libs matched to none"),
    };

    for lib in libs.into_iter() {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

}

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR undefined");
    let out_dir = std::path::PathBuf::from(out_dir);

    #[cfg(feature = "vendored")]
    find_vendored();

    #[cfg(not(feature = "vendored"))]
    find_normal();
}
