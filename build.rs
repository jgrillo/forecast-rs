/*Copyright 2016 Jesse C. Grillo

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.*/

#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new(&out_dir).join("serde_types.rs");

    serde_codegen::expand(&src, &dst).unwrap();
}

#[cfg(not(feature = "serde_codegen"))]
fn main() {
    // do nothing
}
