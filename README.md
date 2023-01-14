# ✒️ enser

Enum Serialization with Tag


## Rationale

When serializing the following enum variants:

```rust
enum WithoutTuple {
    Tbd,
    None,
    Some(u32),
}
```

The output is:

```yaml
# serde_yaml
without_tuple:
- Tbd
- None
- !Some 123

# serde_json
{
  "without_tuple": [
    "Tbd",
    "None",
    { "Some": 123 }
  ]
}
```

When we add an empty tuple to the variants:

```rust
enum WithTuple {
    Tbd(()),
    None(()),
    Some(u32),
}
```

The output is:

```yaml
# serde_yaml -- a !Tag is used for each variant
with_tuple:
- !Tbd null
- !None null
- !Some 123

# serde_json -- every variant is an object
{
  "with_tuple": [
    { "Tbd": null },
    { "None": null },
    { "Some": 123 }
  ]
}
```
