# ✒️ enser

Enum Serialization with Tag


## Usage

Add the following to `Cargo.toml`

```toml
enser = "0.1.0"
```

```diff
  #[derive(Debug, Deserialize, Serialize)]
+ #[enser::enser]
  enum MyEnum {
      Tbd,
      None,
      Some(u32),
      Named { value: u32 },
  }
```


## Rationale

Given the following enum:

```rust
#[derive(Debug, Deserialize, Serialize)]
enum MyEnum {
    Tbd,
    None,
    Some(u32),
    Named { value: u32 },
}
```

When serializing `Vec<MyEnum>`, the output is:

```yaml
# serde_yaml
my_enums:
- Tbd
- None
- !Some 123
- !Named
  value: 456

# serde_json
{
  "without_tuple": [
    "Tbd",
    "None",
    { "Some": 123 }
    { "Named": { "value": 456 } }
  ]
}
```

When the `#[enser::enser]` attribute is added:

```diff
  #[derive(Debug, Deserialize, Serialize)]
+ #[enser::enser]
  enum MyEnum { .. }
```

The output is:

```yaml
# serde_yaml -- a !Tag is used for each variant
my_enums:
- !Tbd null
- !None null
- !Some 123
- !Named
  value: 456

# serde_json -- every variant is an object
{
  "my_enums": [
    { "Tbd": null },
    { "None": null },
    { "Some": 123 },
    { "Named": { "value": 456 } }
  ]
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
