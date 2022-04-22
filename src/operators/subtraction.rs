use serde_json::{Number, Value};

use super::{logic, Data, Expression};

/// "-", takes two numbers and returns the substraction of the them.
/// If only one argument is passed, returns the negation of that argument.
/// Returns `Value::Null` one of the arguments cannot be coerced into a number.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let mut result = args
        .first()
        .and_then(|v| logic::coerce_to_f64(&v.compute(data)))
        .unwrap_or(0.0);

    for arg in args.iter().skip(1) {
        // Use parseFloat like in the javascript implementation.
        // parseFloat(null) is NaN, whereas coerce_to_f64 would return 0.
        match logic::coerce_to_f64(&arg.compute(data)) {
            Some(num) => result -= num,
            None => return Value::Null,
        }
    }

    Value::Number(Number::from_f64(result).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn substraction() {
        assert_eq!(compute_const!(json!(1), json!(2)), json!(-1.0));
        assert_eq!(compute_const!(json!(4), json!(2)), json!(2.0));
        assert_eq!(compute_const!(json!(4), json!(-2)), json!(6.0));
    }
}
