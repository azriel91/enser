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
    let s = Struct {
        without_tuple: vec![
            WithoutTuple::Tbd,
            WithoutTuple::None,
            WithoutTuple::Some(123),
        ],
        with_tuple: vec![
            WithTuple::Tbd(()),
            WithTuple::None(()),
            WithTuple::Some(123),
        ],
        enser: vec![Enser::Tbd, Enser::None, Enser::Some(123)],
    };

    let yaml = serde_yaml::to_string(&s).unwrap();
    println!("{yaml}");

    let json = serde_json::to_string_pretty(&s)
        .unwrap()
        .replace("\n    {\n      ", "\n    { ")
        .replace("\n    }", " }");
    println!("{json}");

    // Output:
    //
    // ```yaml
    // without_tuple:
    // - Tbd
    // - None
    // - !Some 123
    // with_tuple:
    // - !Tbd null
    // - !None null
    // - !Some 123
    // enser:
    // - Tbd
    // - None
    // - !Some 123
    //
    // {
    //   "without_tuple": [
    //     "Tbd",
    //     "None",
    //     { "Some": 123 }
    //   ],
    //   "with_tuple": [
    //     { "Tbd": null },
    //     { "None": null },
    //     { "Some": 123 }
    //   ],
    //   "enser": [
    //     "Tbd",
    //     "None",
    //     { "Some": 123 }
    //   ]
    // }
    // ```
}
