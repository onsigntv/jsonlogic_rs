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
        (a, b) => Value::Bool(logic::greater_equal_than(&a, &b)),
    }
}
