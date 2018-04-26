use serde_json::Value;
use structs::{Criterion, StackItem};
use iter::Iter;

pub fn filter<'a>(pattern: &Criterion, value: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *pattern {
        Criterion::Equal => is_equal(value, values, &root),
        Criterion::Different => is_different(value, values, &root),
        Criterion::Lower => is_lower(value, values, &root),
        Criterion::LowerOrEqual => is_lower_or_equal(value, values, &root),
        Criterion::Greater => is_greater(value, values, &root),
        Criterion::GreaterOrEqual => is_greater_or_equal(value, values, &root),
        _ => None,
    }
}

fn is_equal<'a>(value: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content != content {
                        return Some(false);
                    }
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::Array(ref content) => {
            for item in content {
                if let Some(true) = is_equal(item, values, &root) {
                    return Some(true);
                }
            }
            Some(false)
        }
        Criterion::SubExpression(ref expression) => {
            let found: Vec<&Value> = Iter::new(root.item.value, &expression).collect();

            for item in &found {
                for value in values.iter() {
                    match (*value, *item) {
                        (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                            match (value_content.as_f64(), item_content.as_f64()) {
                                (Some(value_number), Some(item_number)) => {
                                    if value_number != item_number {
                                        return Some(false);
                                    }
                                },
                                _ => {
                                    return Some(false);
                                }
                            }
                        }
                        (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                            if value_content != item_content {
                                return Some(false);
                            }
                        }
                        _ => {
                            return Some(false);
                        }
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn is_different<'a>(value: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content == content {
                        return Some(false);
                    }
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::Array(ref content) => {
            for item in content {
                if let Some(true) = is_equal(item, values, &root) {
                    return Some(true);
                }
            }
            Some(false)
        }
        Criterion::SubExpression(ref expression) => {
            let found: Vec<&Value> = Iter::new(root.item.value, &expression).collect();

            for item in &found {
                for value in values.iter() {
                    match (*value, *item) {
                        (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                            match (value_content.as_f64(), item_content.as_f64()) {
                                (Some(value_number), Some(item_number)) => {
                                    if value_number == item_number {
                                        return Some(false);
                                    }
                                },
                                _ => {
                                    return Some(false);
                                }
                            }
                        }
                        (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                            if value_content == item_content {
                                return Some(false);
                            }
                        }
                        _ => {
                            return Some(false);
                        }
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn is_lower<'a>(value: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content >= content {
                        return Some(false);
                    }
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::SubExpression(ref expression) => {
            let found: Vec<&Value> = Iter::new(root.item.value, &expression).collect();

            for item in &found {
                for value in values.iter() {
                    match (*value, *item) {
                        (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                            match (value_content.as_f64(), item_content.as_f64()) {
                                (Some(value_number), Some(item_number)) => {
                                    if value_number >= item_number {
                                        return Some(false);
                                    }
                                },
                                _ => {
                                    return Some(false);
                                }
                            }
                        }
                        (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                            if value_content >= item_content {
                                return Some(false);
                            }
                        }
                        _ => {
                            return Some(false);
                        }
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn is_lower_or_equal<'a>(value: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *value {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content > content {
                        return Some(false);
                    }
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::Number(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() > Some(*content as f64) {
                        return Some(false);
                    }
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::Float(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() > Some(*content) {
                        return Some(false);
                    }
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::SubExpression(ref expression) => {
            let found: Vec<&Value> = Iter::new(root.item.value, &expression).collect();

            for item in &found {
                for value in values.iter() {
                    match (*value, *item) {
                        (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                            match (value_content.as_f64(), item_content.as_f64()) {
                                (Some(value_number), Some(item_number)) => {
                                    if value_number > item_number {
                                        return Some(false);
                                    }
                                },
                                _ => {
                                    return Some(false);
                                }
                            }
                        }
                        (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                            if value_content > item_content {
                                return Some(false);
                            }
                        }
                        _ => {
                            return Some(false);
                        }
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn is_greater<'a>(criter: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *criter {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content <= content {
                        return Some(false);
                    }
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
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
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::SubExpression(ref expression) => {
            let found: Vec<&Value> = Iter::new(root.item.value, &expression).collect();

            for item in &found {
                for value in values.iter() {
                    match (*value, *item) {
                        (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                            match (value_content.as_f64(), item_content.as_f64()) {
                                (Some(value_number), Some(item_number)) => {
                                    if value_number <= item_number {
                                        return Some(false);
                                    }
                                },
                                _ => {
                                    return Some(false);
                                }
                            }
                        }
                        (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                            if value_content <= item_content {
                                return Some(false);
                            }
                        }
                        _ => {
                            return Some(false);
                        }
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn is_greater_or_equal<'a>(criter: &Criterion, values: &[&Value], root: &StackItem<'a>) -> Option<bool> {
    match *criter {
        Criterion::Literal(ref content) => {
            for v in values.iter() {
                if let Value::String(ref string_content) = **v {
                    if string_content < content {
                        return Some(false);
                    }
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::Number(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() < Some(*content as f64) {
                        return Some(false);
                    }
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::Float(ref content) => {
            for v in values.iter() {
                if let Value::Number(ref number_content) = **v {
                    if number_content.as_f64() < Some(*content) {
                        return Some(false);
                    }
                } else {
                    return Some(false);
                }
            }
            Some(true)
        }
        Criterion::SubExpression(ref expression) => {
            let found: Vec<&Value> = Iter::new(root.item.value, &expression).collect();

            for item in &found {
                for value in values.iter() {
                    match (*value, *item) {
                        (&Value::Number(ref value_content), &Value::Number(ref item_content)) => {
                            match (value_content.as_f64(), item_content.as_f64()) {
                                (Some(value_number), Some(item_number)) => {
                                    if value_number < item_number {
                                        return Some(false);
                                    }
                                },
                                _ => {
                                    return Some(false);
                                }
                            }
                        }
                        (&Value::String(ref value_content), &Value::String(ref item_content)) => {
                            if value_content < item_content {
                                return Some(false);
                            }
                        }
                        _ => {
                            return Some(false);
                        }
                    }
                }
            }
            Some(true)
        }
        _ => None,
    }
}
