//! Enum Serialization with Tag
//!
//! ```diff
//!  # serde_yaml -- every variant starts with a !Tag
//!  enser:
//! -- Tbd
//! -- None
//! +- !Tbd null
//! +- !None null
//!  - !Some 123
//!  - !Named
//!    value: 456
//!
//!  # serde_json -- every variant is an object
//!  {
//!    "enser": [
//! -    "Tbd",
//! -    "None",
//! +    { "Tbd": null },
//! +    { "None": null },
//!      { "Some": 123 },
//!      { "Named": { "value": 456 } }
//!    ]
//!  }
//! ```
//!
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! enser = "0.1.1"
//! ```
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! #
//! #[enser::enser] // <-- just add this
//! // Note: It *must* come above `#[derive(Clone, Deserialize, Serialize)]`
//! #[derive(Clone, Debug, Deserialize, Serialize)]
//! enum MyEnum {
//!     Tbd,
//!     None,
//!     Some(u32),
//!     Named { value: u32 },
//! }
//! ```

pub use enser_derive::enser;
