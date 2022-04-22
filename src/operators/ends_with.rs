use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    match (a, b) {
        (Value::Null, _) | (_, Value::Null) => Value::Bool(false),
        (a, b) => Value::Bool(logic::coerce_to_str(&a).ends_with(&logic::coerce_to_str(&b))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), Value::Bool(false));
        assert_eq!(compute_const!(json!("test-string")), Value::Bool(false));
        assert_eq!(
            compute_const!(json!("This is a sample test"), json!("a sample test")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("This is a sample test"), json!("not a sample test")),
            Value::Bool(false)
        );
        assert_eq!(
            compute_const!(json!("This is a sample test"), json!("This is a")),
            Value::Bool(false)
        );
        assert_eq!(
            compute_const!(json!("This is a sample test"), json!("")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("😀 emoji test 😀"), json!("test 😀")),
            Value::Bool(true)
        );
    }
}
