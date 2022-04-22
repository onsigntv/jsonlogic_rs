use serde_json::Value;

use super::{logic_geo, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    Value::Bool(logic_geo::is_within_regions(a, b))
}

#[cfg(tet)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    fn polygon_geo() {
        assert_eq!(
            compute_const!(
                json!({">.<": [{"var": "_loc"}, {"var": "_geo"}]}),
                json!({
                    "_loc": {"lat": -27.50836, "lng": -48.48099},
                    "_geo": [
                      {"lat": -27.5813671, "lng": -48.48469, "rad": 300},
                      {"path": [
                        [10, 10], [11, 12], [12, 10], [15, 10], [13, 9], [14, 7], [11, 8], [8, 7], [9, 9], [7, 10]
                      ]}
                    ]
                }),
            ),
            json!(false)
        );
    }
}
