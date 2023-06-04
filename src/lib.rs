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
//! enser = "0.1.4"
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
//! # fn main() {}
//! ```
//!
//!
//! ## Generics
//!
//! This will automatically work for generic types:
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! #
//! #[enser::enser]
//! #[derive(Clone, Debug, Deserialize, Serialize)]
//! enum MyEnum<T, U> {
//!     None,
//!     Some(T),
//!     Named { value: U },
//! }
//! # fn main() {}
//! ```
//!
//! **However**, it also adds a `Clone` bound to each type parameter, so all
//! `impl` blocks will require the type parameters to have a `Clone` bound.
//!
//! If you can find a way for the `generics` example to work without causing the
//! `Clone` bound propagation, then please let me know / submit a pull request!

pub use enser_derive::enser;
