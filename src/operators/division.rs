use serde_json::{json, Number, Value};

use super::{logic, Data, Expression};

/// "/", takes two arguments that are coerced into numbers. Returns `Value::Null` if the divisor is
/// coerced to `0` or one argument cannot be coerced into a number.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args
        .get(0)
        .map(|arg| arg.compute(data))
        .and_then(|a| logic::coerce_to_f64(&a))
    {
        Some(a) => a,
        None => return json!(0.0),
    };

    let b = match args
        .get(1)
        .map(|arg| arg.compute(data))
        .and_then(|b| logic::coerce_to_f64(&b))
    {
        Some(b) => b,
        None => return json!(a),
    };

    match Number::from_f64(a / b) {
        Some(num) => Value::Number(num),
        None => json!(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn null() {
        assert_eq!(compute_const!(), json!(0.0));
        assert_eq!(compute_const!(json!("a")), json!(0.0));
        assert_eq!(compute_const!(json!(1)), json!(1.0));
        assert_eq!(compute_const!(json!(1), json!(0)), json!(0.0));

        assert_eq!(compute_const!(json!(1), json!(2)), json!(0.5));
    }
}
