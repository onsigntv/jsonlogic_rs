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

    let result = match (
        serde_json::from_value::<[f64; 2]>(point),
        serde_json::from_value::<[f64; 4]>(rect),
    ) {
        (Ok(point), Ok(rect)) => match args.get(2) {
            Some(refs) => {
                if let Ok(refs) = serde_json::from_value::<Vec<MappingRect>>(refs.compute(data)) {
                    refs.iter()
                        .any(|v| rect_contains_point(point, v.map_rect(rect)))
                } else {
                    false
                }
            }
            None => rect_contains_point(point, rect),
        },
        _ => false,
    };
    Value::Bool(result)
}

fn rect_contains_point(point: [f64; 2], rect: [f64; 4]) -> bool {
    point[0] >= rect[0]
        && point[0] <= rect[0] + rect[2]
        && point[1] >= rect[1]
        && point[1] <= rect[1] + rect[3]
}
