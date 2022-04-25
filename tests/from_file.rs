use std::fs;
use std::path::Path;

use serde_json;
use serde_json::{json, Value};

use jsonlogic::apply;

#[test]
fn run_file_tests() {
    let tests_dir_str = option_env!("CARGO_MANIFEST_DIR")
        .map(|v| format!("{}/tests/json", v))
        .unwrap_or(String::from("./json"));
    let tests_dir = Path::new(&tests_dir_str);
    for entry in tests_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let file = fs::File::open(entry.path()).unwrap();
            let value = serde_json::from_reader(file).unwrap();
            run_json_tests(value, entry.file_name().into_string().unwrap());
        }
    }
}

fn run_json_tests(value: Value, filename: String) {
    if let Value::Array(lines) = value {
        let mut category = String::from("");
        for line in lines.into_iter() {
            match line {
                Value::String(s) => category = s,
                Value::Array(a) => {
                    let [logic, data, expected]: [Value; 3] = a.try_into().unwrap();
                    let result = apply(&logic, &data).unwrap();

                    assert_eq!(
                        value_normalize(result),
                        value_normalize(expected),
                        "\n{}: {}, {}\ntests/json/{}\n",
                        category,
                        logic.to_string(),
                        data.to_string(),
                        filename,
                    )
                }
                _ => {}
            }
        }
    }
}

fn value_normalize(v: Value) -> Value {
    match v {
        Value::Number(n) => json!(n.as_f64()),
        Value::Array(a) => Value::Array(a.into_iter().map(|v| value_normalize(v)).collect()),
        _ => v,
    }
}
