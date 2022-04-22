use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = args
        .get(0)
        .map(|arg| arg.compute(data))
        .unwrap_or(Value::Null);
    let b = args
        .get(1)
        .map(|arg| arg.compute(data))
        .unwrap_or(Value::Null);

    Value::Bool(logic::is_strict_equal(&a, &b))
}
