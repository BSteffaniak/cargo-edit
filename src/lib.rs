//! Show and Edit Cargo's Manifest Files
#![cfg_attr(test, allow(dead_code))]
#![warn(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications
)]

extern crate cargo_metadata;
extern crate env_proxy;
#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate reqwest;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate termcolor;
extern crate toml_edit;

mod crate_name;
mod dependency;
mod errors;
mod fetch;
mod manifest;

pub use crate::crate_name::CrateName;
pub use crate::dependency::Dependency;
pub use crate::errors::*;
pub use crate::fetch::{get_crate_name_from_github, get_crate_name_from_gitlab, get_crate_name_from_path,
                get_latest_dependency};
pub use crate::manifest::{find, LocalManifest, Manifest};
