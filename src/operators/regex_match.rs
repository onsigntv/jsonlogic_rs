use regex::Regex;
use serde_json::Value;

use super::{logic, Data, Expression};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Null,
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Null,
    };

    let mut pattern = logic::coerce_to_str(&b);
    let mut global = false;
    if let Some(Value::String(mut flags)) = args.get(2).map(|v| v.compute(data)) {
        if let Some(g) = flags.find("g") {
            flags.remove(g);
            global = true;
        };
        if flags.len() > 0 {
            pattern = format!("(?{}){}", flags, pattern);
        }
    }

    match Regex::new(&pattern) {
        Ok(re) => {
            let text = &logic::coerce_to_str(&a);
            let results: Vec<&str>;
            if global {
                results = re.find_iter(text).map(|v| v.as_str()).collect();
            } else {
                results = re
                    .captures_iter(text)
                    .flat_map(|v| {
                        v.iter()
                            .filter_map(|v| v)
                            .map(|v| v.as_str())
                            .collect::<Vec<&str>>()
                    })
                    .collect();
            }
            if results.len() > 0 {
                serde_json::to_value(results).unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        }
        Err(_) => Value::Null,
    }
}
