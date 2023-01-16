# ✒️ enser

[![Crates.io](https://img.shields.io/crates/v/enser.svg)](https://crates.io/crates/enser)
[![docs.rs](https://img.shields.io/docsrs/enser)](https://docs.rs/enser)
[![CI](https://github.com/azriel91/enser/workflows/CI/badge.svg)](https://github.com/azriel91/enser/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/enser/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/enser)

Enum Serialization with Tag

```diff
 # serde_yaml -- every variant starts with a !Tag
 enser:
-- Tbd
-- None
+- !Tbd null
+- !None null
 - !Some 123
 - !Named
   value: 456

 # serde_json -- every variant is an object
 {
   "enser": [
-    "Tbd",
-    "None",
+    { "Tbd": null },
+    { "None": null },
     { "Some": 123 },
     { "Named": { "value": 456 } }
   ]
 }
```


## Usage

Add the following to `Cargo.toml`

```toml
enser = "0.1.1"
```

```rust
#[enser::enser] // <-- just add this
                // Note: It *must* come above `#[derive(Clone, Deserialize, Serialize)]`
#[derive(Clone, Debug, Deserialize, Serialize)]
enum MyEnum {
    Tbd,
    None,
    Some(u32),
    Named { value: u32 },
}
```


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
