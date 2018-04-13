use serde_json::Value;
use structs::Criterion;

pub fn filter(pattern: &Criterion, value: &Criterion, values: &[&Value]) -> Option<bool> {
    match *pattern {
        Criterion::Equal => is_equal(value, values),
        Criterion::Different => is_different(value, values),
        Criterion::Lower => is_lower(value, values),
        Criterion::Greater => is_greater(value, values),
        _ => None,
    }
}

fn is_equal(value: &Criterion, values: &[&Value]) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content != content {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Number(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_i64() != Some(*content) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Float(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() != Some(*content) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Array(ref content) => {
            for item in content {
                if let Some(true) = is_equal(item, values) {
                    return Some(true);
                }
            }
            Some(false)
        }
        _ => None,
    }
}

fn is_different(value: &Criterion, values: &[&Value]) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content == content {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Number(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_i64() == Some(*content) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Float(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() == Some(*content) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Array(ref content) => {
            for item in content {
                if let Some(true) = is_equal(item, values) {
                    return Some(true);
                }
            }
            Some(false)
        }
        _ => None,
    }
}

fn is_lower(value: &Criterion, values: &[&Value]) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content >= content {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Number(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() >= Some(*content as f64) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Float(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() >= Some(*content) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn is_greater(value: &Criterion, values: &[&Value]) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content <= content {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Number(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() <= Some(*content as f64) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        Criterion::Float(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() <= Some(*content) {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}
