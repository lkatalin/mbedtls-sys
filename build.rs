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

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR undefined");
    let out_dir = std::path::PathBuf::from(out_dir);

    let bindings = bindgen::builder()
        .detect_include_paths(true)
        .header("src/mbedtls.h")
        .whitelist_function("^mbedtls_.*")
        .whitelist_type("^mbedtls_.*")
        .whitelist_var("^MBEDTLS_.*")
        .whitelist_var("^mbedtls_.*")
        .generate()
        .expect("Error generating bindings!");

    bindings
        .write_to_file(out_dir.join("lib.rs"))
        .expect("Error writing output file!");

    println!("cargo:rustc-link-lib=mbedtls");
    println!("cargo:rustc-link-lib=mbedx509");
    println!("cargo:rustc-link-lib=mbedcrypto");
}
