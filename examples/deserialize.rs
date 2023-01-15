use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Struct {
    without_tuple: Vec<WithoutTuple>,
    with_tuple: Vec<WithTuple>,
    enser: Vec<Enser>,
}

#[derive(Debug, Deserialize, Serialize)]
enum WithoutTuple {
    Tbd,
    None,
    Some(u32),
}

#[derive(Debug, Deserialize, Serialize)]
enum WithTuple {
    Tbd(()),
    None(()),
    Some(u32),
}

#[derive(Debug, Deserialize, Serialize)]
#[enser::enser]
enum Enser {
    Tbd,
    None,
    Some(u32),
}

fn main() {
    let from_yaml: Struct = serde_yaml::from_str(
        r#"without_tuple:
- Tbd
- None
- !Some 123
with_tuple:
- !Tbd null
- !None null
- !Some 123
enser:
- Tbd
- None
- !Some 123"#,
    )
    .unwrap();
    println!("{from_yaml:?}");

    let from_json: Struct = serde_json::from_str(
        r#"{
  "without_tuple": [
    "Tbd",
    "None",
    { "Some": 123 }
  ],
  "with_tuple": [
    { "Tbd": null },
    { "None": null },
    { "Some": 123 }
  ],
  "enser": [
    "Tbd",
    "None",
    { "Some": 123 }
  ]
}
"#,
    )
    .unwrap();
    println!("{from_json:?}");
}
