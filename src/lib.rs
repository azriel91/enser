//! Enum Serialization with Tag
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! enser = "0.1.1"
//! ```
//!
//! ```diff
//!   #[derive(Debug, Deserialize, Serialize)]
//! + #[enser::enser]
//!   enum MyEnum {
//!       Tbd,
//!       None,
//!       Some(u32),
//!       Named { value: u32 },
//!   }
//! ```
//!
//! ## Rationale
//!
//! Given the following enum:
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! #
//! #[derive(Debug, Deserialize, Serialize)]
//! enum MyEnum {
//!     Tbd,
//!     None,
//!     Some(u32),
//!     Named { value: u32 },
//! }
//! ```
//!
//! When serializing `Vec<MyEnum>`, the output is:
//!
//! ```yaml
//! # serde_yaml
//! my_enums:
//! - Tbd
//! - None
//! - !Some 123
//! - !Named
//!   value: 456
//!
//! # serde_json
//! {
//!   "without_tuple": [
//!     "Tbd",
//!     "None",
//!     { "Some": 123 }
//!     { "Named": { "value": 456 } }
//!   ]
//! }
//! ```
//!
//! When the `#[enser::enser]` attribute is added:
//!
//! ```diff
//!   #[derive(Debug, Deserialize, Serialize)]
//! + #[enser::enser]
//!   enum MyEnum { .. }
//! ```
//!
//! The output is:
//!
//! ```yaml
//! # serde_yaml -- a !Tag is used for each variant
//! my_enums:
//! - !Tbd null
//! - !None null
//! - !Some 123
//! - !Named
//!   value: 456
//!
//! # serde_json -- every variant is an object
//! {
//!   "my_enums": [
//!     { "Tbd": null },
//!     { "None": null },
//!     { "Some": 123 },
//!     { "Named": { "value": 456 } }
//!   ]
//! }
//! ```

pub use enser_derive::enser;
