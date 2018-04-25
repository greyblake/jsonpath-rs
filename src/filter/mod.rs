use serde_json::Value;
use structs::{Criterion, StackItem};
use iter::Iter;

mod comparison;

pub fn process_filter<'a>(stack: &mut StackItem, path: &[Criterion], root: &StackItem<'a>) -> bool {
    let mut iterator = path.iter();
    match iterator.next() {
        Some(&Criterion::Element) => {
            let found_condition = path.iter().position(|x| {
                x == &Criterion::Equal || x == &Criterion::Different || x == &Criterion::Greater
                    || x == &Criterion::GreaterOrEqual || x == &Criterion::Lower || x == &Criterion::LowerOrEqual
            });

            let (sub_path, condition) = match found_condition {
                Some(index) => {
                    let (left, right) = path.split_at(index);
                    (left[1..].to_vec(), right.to_vec())
                }
                None => (path.to_owned(), vec![]),
            };

            let mut full_criterion = vec![Criterion::Root];
            full_criterion.extend_from_slice(&sub_path);

            let found: Vec<&Value> = Iter::new(stack.item.value, &full_criterion).collect();

            match condition.len() {
                0 => !found.is_empty(),
                2 => match comparison::filter(&condition[0], &condition[1], &found) {
                    Some(value) => value,
                    None => false,
                },
                _ => false,
            }
        }
        Some(&Criterion::Root) => {
            let found = path.iter().position(|x| {
                x == &Criterion::Equal || x == &Criterion::Different || x == &Criterion::Greater
                    || x == &Criterion::Lower
            });

            let (sub_path, condition) = match found {
                Some(index) => {
                    let (left, right) = path.split_at(index);
                    (left.to_vec(), right.to_vec())
                }
                None => (path.to_owned(), vec![]),
            };

            let doc = root.item.value;
            let found: Vec<&Value> = Iter::new(doc, &sub_path).collect();

            match condition.len() {
                0 => !found.is_empty(),
                2 => match comparison::filter(&condition[0], &condition[1], &found) {
                    Some(value) => value,
                    None => false,
                },
                _ => false,
            }
        }
        _ => false,
    }
}
