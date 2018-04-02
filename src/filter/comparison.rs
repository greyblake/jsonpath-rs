use serde_json::Value;
use structs::{Criterion, StackItem};

macro_rules! numbers {
    (integer => $next:expr, $operator:tt, $value:expr) => {{
        match *$next.item.value {
            Value::Number(ref source) => {
                match source.as_f64() {
                    Some(ref float_value) => Some(*float_value $operator *$value as f64),
                    None => {
                        match source.as_i64() {
                            Some(ref int_value) => Some(int_value $operator $value),
                            None => None
                        }
                    }
                }
            },
            _ => None,
        }
    }};
    (float => $next:expr, $operator:tt, $value:expr) => {{
        match *$next.item.value {
            Value::Number(ref source) => {
                match source.as_f64() {
                    Some(ref float_value) => Some(float_value $operator ($value)),
                    None => {
                        match source.as_i64() {
                            Some(ref int_value) => Some((*int_value as f64) $operator *$value),
                            None => None
                        }
                    }
                }
            },
            _ => None,
        }
    }}
}

pub fn filter(pattern: &Criterion, value: &Criterion, next: &StackItem) -> Option<bool> {
    match (pattern, value) {
        (&Criterion::Equal, &Criterion::Literal(ref content)) => Some(next.item.value == content),
        (&Criterion::Different, &Criterion::Literal(ref content)) => {
            Some(next.item.value != content)
        }
        (&Criterion::Lower, &Criterion::Number(ref value)) => numbers!(integer => next, <, value),
        (&Criterion::Greater, &Criterion::Number(ref value)) => numbers!(integer => next, >, value),
        (&Criterion::Lower, &Criterion::Float(ref value)) => numbers!(float => next, <, value),
        (&Criterion::Greater, &Criterion::Float(ref value)) => numbers!(float => next, >, value),
        _ => None,
    }
}

pub fn vec_filter(pattern: &Criterion, value: &Criterion, values: &[&Value]) -> Option<bool> {
    match (pattern, value) {
        (&Criterion::Equal, &Criterion::Literal(ref content)) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content != content {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        (&Criterion::Different, &Criterion::Literal(ref content)) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content == content {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }
        (&Criterion::Lower, &Criterion::Number(ref _value)) => unimplemented!(),
        (&Criterion::Greater, &Criterion::Number(ref _value)) => unimplemented!(),
        (&Criterion::Lower, &Criterion::Float(ref _value)) => unimplemented!(),
        (&Criterion::Greater, &Criterion::Float(ref _value)) => unimplemented!(),
        _ => None,
    }
}
