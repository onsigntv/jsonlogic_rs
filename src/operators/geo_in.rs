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

    Value::Bool(logic_geo::is_within_region(a, b))
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
                    "_loc": {"lng": 2, "lat": 2},
                    "_geo": {"path": [[0, 0], [5, 0], [5, 5], [0, 5]]}
                }),
            ),
            json!(true)
        );
        assert_eq!(
            compute_const!(
                json!({">.<": [{"var": "_loc"}, {"var": "_geo"}]}),
                json!({
                    "_loc": {"lng": 2, "lat": 2},
                    "_geo": {"path": [
                        [-229.283638, 30.9477275],
                        [-219.5717239, 35.037446],
                        [-218.4291458, 41.053078],
                        [-230.5580521, 33.2554844],
                        [-229.283638, 30.9477275]
                    ]}
                }),
            ),
            json!(false)
        );
    }

    #[test]
    fn circle_geo() {
        assert_eq!(
            compute_const!(
                json!({">.<": [{"var": "_loc"}, {"var": "_geo"}]}),
                json!({
                    "_loc": {"lat": -27.54623, "lng": -48.48590},
                    "_geo": {"lat": -27.5813671, "lng": -48.48469, "rad": 300}
                }),
            ),
            json!(true)
        );
        assert_eq!(
            compute_const!(
                json!({">.<": [{"var": "_loc"}, {"var": "_geo"}]}),
                json!({
                    "_loc": {"lat": -27.50836, "lng": -48.48099},
                    "_geo": {"lat": -27.5813671, "lng": -48.48469, "rad": 300}
                }),
            ),
            json!(false)
        );
    }
}
