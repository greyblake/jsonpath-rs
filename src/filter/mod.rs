
use serde_json::Value;
use structs::{Criterion, StackItem, Step};


macro_rules! numbers {
  (integer => $next:expr, $operator:tt, $value:expr) => {{
    match $next.item.value {
      &Value::Number(ref source) => {
        match source.as_f64() {
          Some(ref float_value) => return *float_value $operator *$value as f64,
          None => {
            match source.as_i64() {
              Some(ref int_value) => return int_value $operator $value,
              None => {}
            }
          }
        }
      },
      _ => {},
    }
  }};
  (float => $next:expr, $operator:tt, $value:expr) => {{
    match $next.item.value {
      &Value::Number(ref source) => {
        match source.as_f64() {
          Some(ref float_value) => return float_value $operator ($value),
          None => {
            match source.as_i64() {
              Some(ref int_value) => return (*int_value as f64) $operator *$value,
              None => {}
            }
          }
        }
      },
      _ => {},
    }
  }}
}

pub fn process_filter(stack: &mut StackItem, path: &Vec<Criterion>) -> bool {
  let step = stack.step.clone();

  match path[0] {
    Criterion::Element => {
      match path[1] {
        Criterion::NamedChild(ref child_name) => {
          while let Some(next) = stack.next() {
            match next.step {
              Step::Key(key) => {
                if child_name == key {
                  match (path[2].clone(), path[3].clone()) {
                    (Criterion::Equal, Criterion::Literal(ref content)) => {
                      return next.item.value == content
                    }
                    (Criterion::Different, Criterion::Literal(ref content)) => {
                      return next.item.value != content
                    }
                    (Criterion::Lower, Criterion::Number(ref value)) => {
                      numbers!(integer => next, <, value)
                    }
                    (Criterion::Greater, Criterion::Number(ref value)) => {
                      numbers!(integer => next, >, value)
                    }
                    (Criterion::Lower, Criterion::Float(ref value)) => {
                      numbers!(float => next, <, value)
                    }
                    (Criterion::Greater, Criterion::Float(ref value)) => {
                      numbers!(float => next, >, value)
                    }
                    _ => {}
                  }
                }
              },
              _ => {},
            }
          }
          false
        },
        _ => false,
      }
    },
    Criterion::Root => {
      step == Step::Root
    },
  _ => false,
  }
}

