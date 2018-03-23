
use serde_json::Value;
use structs::{Criterion, StackItem, Step};
use iter::Iter;

mod comparison;

pub fn process_filter<'a>(stack: &mut StackItem, path: &Vec<Criterion>, root: &StackItem<'a>) -> bool {
  match path[0] {
    Criterion::Element => {
      match path[1] {
        Criterion::NamedChild(ref child_name) => {
          while let Some(next) = stack.next() {
            match next.step {
              Step::Key(key) => {
                if child_name == key {
                  match comparison::filter(&path[2], &path[3], &next) {
                    Some(value) => return value,
                    None => {},
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
      let found =
        path.iter().position(|ref x|
          *x == &Criterion::Equal ||
          *x == &Criterion::Different ||
          *x == &Criterion::Greater ||
          *x == &Criterion::Lower
        );

      let (sub_path, condition) =
        match found {
          Some(index) => {
            let (left, right) = path.split_at(index);
            (left.to_vec(), right.to_vec())
          },
          None => (path.clone(), vec![]),
        };

      let doc = root.item.value;
      let found: Vec<&Value> = Iter::new(&doc, &sub_path).collect();

      match condition.len() {
        0 => true,
        2 => {
          match comparison::vec_filter(&condition[0], &condition[1], &found) {
            Some(value) => return value,
            None => false,
          }
        },
        _ => false
      }
    },
  _ => false,
  }
}

