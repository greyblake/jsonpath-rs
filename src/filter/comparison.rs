use iter::Iter;
use serde_json::Value;
use std::f64::EPSILON;
use structs::{Criterion, StackItem};

pub fn filter<'a>(
    pattern: &Criterion,
    value: &Criterion,
    values: &[&Value],
    root: &StackItem<'a>,
) -> Option<bool> {
    match *pattern {
        Criterion::Equal => is_equal(value, values, root),
        Criterion::Different => is_different(value, values, root),
        Criterion::Lower => is_lower(value, values, root),
        Criterion::LowerOrEqual => is_lower_or_equal(value, values, root),
        Criterion::Greater => is_greater(value, values, root),
        Criterion::GreaterOrEqual => is_greater_or_equal(value, values, root),
        _ => None,
    }
}

macro_rules! compare {
    ($criterion:expr, $values:expr, $root:expr, $operator:tt,
     $number_operator:tt, $eplison:expr, $absolute:expr, $method:tt) => (
        match *$criterion {
            Criterion::Literal(ref content) => {
                for v in $values.iter() {
                    if let Value::String(ref string_content) = **v {
                        if string_content $operator content {
                            return Some(false);
                        }
                    } else {
                        return Some(false);
                    }
                }
                Some(true)
            }

            Criterion::Float(ref content) => {
                for v in $values.iter() {
                    if let Value::Number(ref number_content) = **v {
                        if number_content.as_f64() $operator Some(*content) {
                            return Some(false);
                        }
                    } else {
                        return Some(false);
                    }
                }
                Some(true)
            }
            Criterion::Array(ref content) => {
                for item in content {
                    if let Some(true) = $method(item, $values, &$root) {
                        return Some(true);
                    }
                }
                Some(false)
            }
            Criterion::SubExpression(ref expression) => {
                validate_sub_expresion!($values, $root,
                    $operator, $number_operator, $eplison, $absolute, expression)
            }
            _ => None,
        }
    )
}

macro_rules! validate_sub_expresion {
    ($values:expr, $root:expr, $operator:tt, $number_operator:tt,
     $eplison:expr, $absolute:expr, $expression:expr) => ({
        let found: Vec<&Value> = Iter::new($root.item.value, &$expression).collect();

        for item in &found {
            for value in $values.iter() {
                match (*value, *item) {
                    (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                        match (value_content.as_f64(), item_content.as_f64()) {
                            (Some(value_number), Some(item_number)) => {
                                if !$absolute &&
                                    (value_number - item_number) $number_operator $eplison {
                                    return Some(false);
                                }
                                if $absolute &&
                                    (value_number - item_number).abs() $number_operator $eplison {
                                    return Some(false);
                                }
                            }
                            _ => {
                                return Some(false);
                            }
                        }
                    }
                    (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                        if value_content $operator item_content {
                            return Some(false);
                        }
                    }
                    (&Value::String(ref value_content), &Value::Array(ref items)) => {
                        let mut returned = false;
                        for item in items {
                            if let Value::String(ref item_content) = *item {
                                if !(value_content $operator item_content) {
                                    returned = true;
                                }
                            }
                        }
                        if !returned {
                            return Some(false)
                        }
                    }
                    _ => {
                        return Some(false);
                    }
                }
            }
        }
        Some(true)
    })
}

fn is_equal<'a>(criterion: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    compare!(criterion, values, root, !=, >, EPSILON, true, is_equal)
}

fn is_different<'a>(
    criterion: &Criterion,
    values: &[&Value],
    root: &StackItem<'a>,
) -> Option<bool> {
    compare!(criterion, values, root, ==, <, EPSILON, true, is_different)
}

fn is_lower<'a>(criterion: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    compare!(criterion, values, root, >=, >=, -EPSILON, false, is_lower)
}

fn is_lower_or_equal<'a>(
    value: &Criterion,
    values: &[&Value],
    root: &StackItem<'a>,
) -> Option<bool> {
    compare!(value, values, root, >, >, EPSILON, false, is_lower_or_equal)
}

fn is_greater<'a>(criterion: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    compare!(criterion, values, root, <=, <, EPSILON, false, is_greater)
}

fn is_greater_or_equal<'a>(
    criterion: &Criterion,
    values: &[&Value],
    root: &StackItem<'a>,
) -> Option<bool> {
    compare!(criterion, values, root, <, <, -EPSILON, false, is_greater_or_equal)
}
