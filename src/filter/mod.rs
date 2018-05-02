use serde_json::Value;
use structs::{Criterion, StackItem};
use iter::Iter;

mod comparison;

pub fn process_filter<'a>(stack: &mut StackItem, path: &[Criterion], root: &StackItem<'a>) -> bool {
    let mut or_indexes = vec![];
    let mut and_indexes = vec![];

    for (index, criterion) in path.iter().enumerate() {
        if let &Criterion::Or = criterion {
            or_indexes.push(index);
        }
        if let &Criterion::And = criterion {
            and_indexes.push(index);
        }
    }

    for (index, i) in or_indexes.iter().enumerate() {
        let (left, right) = path.split_at(*i);

        if index != or_indexes.len() - 1 {
            if process_filter(stack, left, root) {
                return true
            }
        } else {
            let mut right_vec = right.to_vec();
            right_vec.remove(0);

            if process_filter(stack, left, root) ||
               process_filter(stack, &right_vec, root) {
                return true
            }
        }
    }
    
    if or_indexes.is_empty() && !and_indexes.is_empty(){
        for (index, i) in and_indexes.iter().enumerate() {
            let (left, right) = path.split_at(*i);

            if index != and_indexes.len() - 1 {
                if !process_filter(stack, left, root) {
                    return false
                }
            } else {
                let mut right_vec = right.to_vec();
                right_vec.remove(0);

                if !process_filter(stack, left, root) ||
                   !process_filter(stack, &right_vec, root) {
                    return false
                }
            }
        }
        return true;
    }

    if !(or_indexes.is_empty() && and_indexes.is_empty()) {
        return false;
    }

    let mut iterator = path.iter();
    match iterator.next() {
        Some(&Criterion::Element) => {
            let found_condition = path.iter().position(|x| {
                x == &Criterion::Equal || x == &Criterion::Different || x == &Criterion::Greater
                    || x == &Criterion::GreaterOrEqual || x == &Criterion::Lower
                    || x == &Criterion::LowerOrEqual
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
                2 => match comparison::filter(&condition[0], &condition[1], &found, root) {
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

            let found: Vec<&Value> = Iter::new(root.item.value, &sub_path).collect();

            match condition.len() {
                0 => !found.is_empty(),
                2 => match comparison::filter(&condition[0], &condition[1], &found, root) {
                    Some(value) => value,
                    None => false,
                },
                _ => false,
            }
        }
        _ => false,
    }
}
