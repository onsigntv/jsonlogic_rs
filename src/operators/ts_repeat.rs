use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let ts = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let start = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let freq = match args.get(2) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    match (
        logic::coerce_to_f64(&ts),
        logic::coerce_to_f64(&start),
        logic::coerce_to_f64(&freq),
    ) {
        (Some(ts), Some(start), Some(freq)) => Value::Bool(((ts % 86400.0) - start) % freq == 0.0),
        _ => Value::Bool(false),
    }
}
