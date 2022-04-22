use serde_json::{json, Value};

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let ts = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return json!(0.0),
    };

    let start = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return json!(0.0),
    };

    let freq = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return json!(0.0),
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

#[cfg(tet)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    fn touch() {
        assert_eq!(
            compute_const!(
                json!({"tsrep": [{"var": "_ts"}, 37200, 600]}),
                json!({"_ts": 1588598400}),
            ),
            json!(true)
        );
        assert_eq!(
            compute_const!(
                json!({"tsrep": [{"var": "_ts"}, 37200, 600]}),
                json!({"_ts": 1588598435}),
            ),
            json!(false)
        );
        assert_eq!(
            compute_const!(
                json!({"tsrep": [{"var": "_ts"}, 37200, 60]}),
                json!({"_ts": 1588598460}),
            ),
            json!(true)
        );
        assert_eq!(
            compute_const!(
                json!({"tsrep": [{"var": "_ts"}, 0, 60]}),
                json!({"_ts": 1588598460}),
            ),
            json!(true)
        );
    }
}
