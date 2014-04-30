//! Common HTTP functionality for the Teepee project.
//!
//! Anything shared between both client and server belongs in here, but this crate is not expected
//! to be used directly.
//!
//! Any crate using types from this crate should re‐export them. For example, the ``status`` module
//! should be exported in the root of the HTTP client crate ``httpc`` so that people can write
//! ``httpc::status`` instead of ``httpcommon::status``.

#![crate_id = "httpcommon#0.1-pre"]
#![comment = "Common HTTP functionality for the Teepee project"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![doc(html_root_url = "http://www.rust-ci.org/teepee/httpcommon/doc/")]

#![deny(unnecessary_qualification)]
#![deny(non_uppercase_statics)]
#![deny(unnecessary_typecast)]
#![deny(missing_doc)]
//#![deny(unstable)]
#![deny(unused_result)]
#![deny(deprecated_owned_vector)]

pub mod status;