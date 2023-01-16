use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Struct {
    enser_1: Vec<Enser1<u8, u16>>,
    enser_2: Vec<Enser2<u8, u16>>,
    enser_3: Vec<Enser3<u8, u16>>,
    enser_4: Vec<Enser4<u8, u16>>,
}

trait Trait {}

impl Trait for u8 {}
impl Trait for u16 {}

// No trait bounds
#[enser::enser]
#[derive(Clone, Debug, Deserialize, Serialize)]
enum Enser1<T, U> {
    None,
    Some(T),
    Named { value: U },
}

// Trait bounds without Clone
#[enser::enser]
#[derive(Clone, Debug, Deserialize, Serialize)]
enum Enser2<T, U>
where
    T: Trait,
{
    None,
    Some(T),
    Named { value: U },
}

// Trait bounds with Clone
#[enser::enser]
#[derive(Clone, Debug, Deserialize, Serialize)]
enum Enser3<T, U>
where
    T: Trait + Clone,
    U: Trait + Clone,
{
    None,
    Some(T),
    Named { value: U },
}

// Multiple trait bounds without Clone
#[enser::enser]
#[derive(Clone, Debug, Deserialize, Serialize)]
enum Enser4<T, U>
where
    T: Trait,
    T: 'static,
{
    None,
    Some(T),
    Named { value: U },
}

impl<T, U> std::fmt::Display for Enser4<T, U>
where
    // Note: These require T and U to be `Clone`, which are
    // added by the proc macro to the original enum.
    T: Clone + std::fmt::Display + Trait,
    U: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => "None".fmt(f),
            Self::Some(t) => t.fmt(f),
            Self::Named { value } => value.fmt(f),
        }
    }
}

fn main() {
    let s = Struct {
        enser_1: vec![
            Enser1::None,
            Enser1::Some(123),
            Enser1::Named { value: 456 },
        ],
        enser_2: vec![
            Enser2::None,
            Enser2::Some(123),
            Enser2::Named { value: 456 },
        ],
        enser_3: vec![
            Enser3::None,
            Enser3::Some(123),
            Enser3::Named { value: 456 },
        ],
        enser_4: vec![
            Enser4::None,
            Enser4::Some(123),
            Enser4::Named { value: 456 },
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
    // enser_1:
    // - None
    // - !Some 123
    // - !Named
    //   value: 456
    //
    // {
    //   "enser_1": [
    //     "None",
    //     { "Some": 123 },
    //     { "Named": { "value": 456 } }
    //   ]
    // }
    // ```
}
