use serde::Deserialize;
use serde_json;
use serde_json::Value;

use super::{Data, Expression};

#[derive(Deserialize)]
struct MappingRect {
    rect: [f64; 4],
}

impl MappingRect {
    fn map_rect(&self, rect: [f64; 4]) -> [f64; 4] {
        [
            self.rect[0] + (self.rect[2] * rect[0]) / 100000.0,
            self.rect[1] + (self.rect[3] * rect[1]) / 100000.0,
            (self.rect[2] * rect[2]) / 100000.0,
            (self.rect[3] * rect[3]) / 100000.0,
        ]
    }
}

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let point = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let rect = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(true),
    };

    match (
        serde_json::from_value::<[f64; 2]>(point),
        serde_json::from_value::<[f64; 4]>(rect),
    ) {
        (Ok(point), Ok(rect)) => {
            if let Some(Ok(mapping_refs)) = args
                .get(0)
                .map(|v| serde_json::from_value::<Vec<MappingRect>>(v.compute(data)))
            {
                return Value::Bool(
                    mapping_refs
                        .iter()
                        .any(|v| rect_contains_point(point, v.map_rect(rect))),
                );
            } else {
                return Value::Bool(rect_contains_point(point, rect));
            }
        }
        _ => return Value::Bool(false),
    }
}

fn rect_contains_point(point: [f64; 2], rect: [f64; 4]) -> bool {
    point[0] >= rect[0]
        && point[0] <= rect[0] + rect[2]
        && point[1] >= rect[1]
        && point[1] <= rect[1] + rect[3]
}

#[cfg(tet)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    fn touch() {
        assert_eq!(
            compute_const!(
                json!({">t<": [
                    {"var": "_touch"},
                    [0, 0, 100000, 75000],
                    {"var": "_playing.nnfnBNJ"}
                ]}),
                json!({"_touch": [50000, 50000], "_playing": {"nnfnBNJ": [{"rect": [0, 0, 100000, 75000]}]}}),
            ),
            json!(true)
        );
        assert_eq!(
            compute_const!(
                json!({">t<": [
                    {"var": "_touch"},
                    [0, 0, 100000, 100000]
                ]}),
                json!({"_touch": [50000, 50000], "_playing": {"nnfnBNJ": [{"rect": [0, 0, 100000, 75000]}]}}),
            ),
            json!(true)
        );
        assert_eq!(
            compute_const!(
                json!({">t<": [
                    {"var": "_touch"},
                    [0, 0, 100000, 75111],
                    {"var": "_playing.nnfnBNJ"}
                ]}),
                json!({"_touch": [50000, 50000], "_playing": {}}),
            ),
            json!(false)
        );
        assert_eq!(
            compute_const!(
                json!({">t<": [
                    {"var": "_touch"},
                    [0, 0, 100000, 75000],
                    {"var": "_playing.nnfnBNJ"}
                ]}),
                json!({"_touch": [99999, 99999], "_playing": {"nnfnBNJ": [{"rect": [0, 0, 100000, 75000]}]}}),
            ),
            json!(false)
        );
        assert_eq!(
            compute_const!(
                json!({">t<": [
                  {"var": "_touch"},
                  [50000, 0, 50000, 100000],
                  {"var": "_playing.Blf6r8x"}
                ]}),
                json!({"_touch": [0, 0], "_playing": {"Blf6r8x": [{"rect": [0, 0, 100000, 100000]}]}}),
            ),
            json!(false)
        );
    }
}
