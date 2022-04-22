use regex::Regex;
use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Null,
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Null,
    };

    let mut pattern = logic::coerce_to_str(&b);
    let mut global = false;
    if let Some(Value::String(mut flags)) = args.get(2).map(|v| v.compute(data)) {
        if let Some(g) = flags.find("g") {
            flags.remove(g);
            global = true;
        };
        if flags.len() > 0 {
            pattern = format!("(?{}){}", flags, pattern);
        }
    }

    match Regex::new(&pattern) {
        Ok(re) => {
            let text = &logic::coerce_to_str(&a);
            let results: Vec<&str>;
            if global {
                results = re.find_iter(text).map(|v| v.as_str()).collect();
            } else {
                results = re
                    .captures_iter(text)
                    .flat_map(|v| {
                        v.iter()
                            .filter_map(|v| v)
                            .map(|v| v.as_str())
                            .collect::<Vec<&str>>()
                    })
                    .collect();
            }
            if results.len() > 0 {
                serde_json::to_value(results).unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        }
        Err(_) => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn regex() {
        assert_eq!(compute_const!(), json!(null));
        assert_eq!(compute_const!(json!("test test-test")), json!(null));
        assert_eq!(
            compute_const!(json!("test test-test"), json!("test")),
            json!(["test", "test", "test"])
        );
        assert_eq!(
            compute_const!(json!("123 456 789"), json!("\\d+")),
            json!(["123", "456", "789"])
        );
        assert_eq!(
            compute_const!(json!("A test"), json!("A.*")),
            json!(["A test"])
        );
        assert_eq!(compute_const!(json!("a test"), json!("A.*")), json!(null));
        assert_eq!(
            compute_const!(json!("a test"), json!("A.*"), json!("i")),
            json!(["a test"])
        );
        assert_eq!(
            compute_const!(json!("A test"), json!("A.*"), json!("g")),
            json!(["A test"])
        );
        assert_eq!(
            compute_const!(json!("A test"), json!("A\\s(.*)")),
            json!(["A test", "test"])
        );
        assert_eq!(
            compute_const!(json!("A test"), json!("A\\s(.*)"), json!("g")),
            json!(["A test"])
        );
    }
}
