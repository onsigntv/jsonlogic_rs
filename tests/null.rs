use jsonlogic::apply;
use serde_json::{json, Value};

#[test]
fn eq_null_args() {
    assert_eq!(
        apply(&json!({"==": [null, false]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"===": [null, false]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"!=": [null, false]}), &Value::Null),
        Ok(Value::Bool(true))
    );
    assert_eq!(
        apply(&json!({"!==": [null, false]}), &Value::Null),
        Ok(Value::Bool(true))
    );
}

#[test]
fn gt_lt_null_args() {
    assert_eq!(
        apply(&json!({">=": [2f64, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({">": [2f64, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"<=": [2f64, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"<": [2f64, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );

    assert_eq!(
        apply(&json!({">=": [null, 2f64]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({">": [null, 2f64]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"<=": [null, 2f64]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"<": [null, 2f64]}), &Value::Null),
        Ok(Value::Bool(false))
    );

    assert_eq!(
        apply(&json!({">=": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({">": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"<=": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"<": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
}

#[test]
fn in_null_args() {
    assert_eq!(
        apply(&json!({"in": [null, [1f64, 2f64, 3f64]]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"in": [1, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"in": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"in": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
}

#[test]
fn op_null_args() {
    assert_eq!(apply(&json!({"+": [1f64]}), &Value::Null), Ok(json!(1f64)));
    assert_eq!(
        apply(&json!({"+": [null, 1f64]}), &Value::Null),
        Ok(json!(1f64))
    );
    assert_eq!(
        apply(&json!({"+": [1f64, null]}), &Value::Null),
        Ok(json!(1f64))
    );
    assert_eq!(
        apply(&json!({"+": [null, null]}), &Value::Null),
        Ok(json!(0.0))
    );

    assert_eq!(apply(&json!({"-": [1f64]}), &Value::Null), Ok(json!(1f64)));
    assert_eq!(
        apply(&json!({"-": [null, 1f64]}), &Value::Null),
        Ok(json!(-1f64))
    );
    assert_eq!(
        apply(&json!({"-": [-1f64, null]}), &Value::Null),
        Ok(json!(-1f64))
    );
    assert_eq!(
        apply(&json!({"-": [null, null]}), &Value::Null),
        Ok(json!(0.0))
    );

    assert_eq!(apply(&json!({"*": [1f64]}), &Value::Null), Ok(json!(1f64)));
    assert_eq!(
        apply(&json!({"*": [null, 1f64]}), &Value::Null),
        Ok(json!(0.0))
    );
    assert_eq!(
        apply(&json!({"*": [1f64, null]}), &Value::Null),
        Ok(json!(0.0))
    );
    assert_eq!(
        apply(&json!({"*": [null, null]}), &Value::Null),
        Ok(json!(0.0))
    );

    assert_eq!(apply(&json!({"/": [1f64]}), &Value::Null), Ok(json!(1f64)));
    assert_eq!(
        apply(&json!({"/": [null, 1f64]}), &Value::Null),
        Ok(json!(0.0))
    );
    assert_eq!(
        apply(&json!({"/": [1f64, null]}), &Value::Null),
        Ok(json!(0.0))
    );
    assert_eq!(
        apply(&json!({"/": [null, null]}), &Value::Null),
        Ok(json!(0.0))
    );

    assert_eq!(
        apply(&json!({"%": [null, 1f64]}), &Value::Null),
        Ok(json!(0.0))
    );
    assert_eq!(
        apply(&json!({"%": [1f64, null]}), &Value::Null),
        Ok(json!(0.0))
    );
    assert_eq!(
        apply(&json!({"%": [null, null]}), &Value::Null),
        Ok(json!(0.0))
    );
}

#[test]
fn string_extra_null_args() {
    assert_eq!(
        apply(&json!({"*=": [null, "text"]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"*=": ["text", null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"*=": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );

    assert_eq!(
        apply(&json!({"=*": [null, "text"]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"=*": ["text", null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
    assert_eq!(
        apply(&json!({"=*": [null, null]}), &Value::Null),
        Ok(Value::Bool(false))
    );
}
