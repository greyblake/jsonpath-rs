use serde_json::Value;
use structs::{Criterion, StackItem, Step};
use iter::Iter;

mod comparison;

pub fn process_filter<'a>(stack: &mut StackItem, path: &[Criterion], root: &StackItem<'a>) -> bool {
    println!("Process Filter: {:?}", path);
    match path[0] {
        Criterion::Element => match path[1] {
            Criterion::NamedChild(ref child_name) => {
                while let Some(next) = stack.next() {
                    if let Step::Key(key) = next.step {
                        if child_name == key {
                            if let Some(value) = comparison::filter(&path[2], &path[3], &next) {
                                return value;
                            }
                        }
                    }
                }
                false
            }
            _ => false,
        },
        Criterion::Root => {
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
                0 => true,
                2 => match comparison::vec_filter(&condition[0], &condition[1], &found) {
                    Some(value) => value,
                    None => false,
                },
                _ => false,
            }
        }
        _ => false,
    }
}
