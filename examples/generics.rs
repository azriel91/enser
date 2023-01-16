use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Struct {
    enser: Vec<Enser<u8, u16>>,
}

trait Trait {}

impl Trait for u8 {}
impl Trait for u16 {}

#[derive(Debug, Deserialize, Serialize)]
#[enser::enser]
enum Enser<T, U>
where
    T: Trait,
    U: Trait,
{
    Tbd,
    None,
    Some(T),
    Named { value: U },
}

fn main() {
    let s = Struct {
        enser: vec![
            Enser::Tbd,
            Enser::None,
            Enser::Some(123),
            Enser::Named { value: 456 },
        ],
    };

    let yaml = serde_yaml::to_string(&s).unwrap();
    println!("{yaml}");

    let json = serde_json::to_string_pretty(&s)
        .unwrap()
        .replace("\n    {\n      ", "\n    { ")
        .replace("\n        \"", " \"")
        .replace("\n      }", " }")
        .replace("\n    }", " }");
    println!("{json}");

    // Output:
    //
    // ```yaml
    // enser:
    // - Tbd
    // - None
    // - !Some 123
    // - !Named
    //   value: 456
    //
    // {
    //   "enser": [
    //     "Tbd",
    //     "None",
    //     { "Some": 123 },
    //     { "Named": { "value": 456 } }
    //   ]
    // }
    // ```
}
