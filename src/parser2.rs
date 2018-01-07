use serde_json::Value;
use structs::{Criterion, Step, StackItem, Item, matches};

struct Iter<'a, 'b> {
    criteria: &'b Vec<Criterion>,
    ci: usize,
    current: Option<StackItem<'a>>,
    stack: Vec<StackItem<'a>>
}

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut current) = self.current.take() {
            if let Some(criterion) = self.criteria.get(self.ci) {
                if matches(&current.step, criterion) {
                    // if there are no further criteria
                    if self.criteria.len() == self.ci + 1 {
                        let val = current.item.value;
                        self.ci -= 1;
                        self.current = self.stack.pop();
                        return Some(val);
                    } else {
                        self.current = current.next();
                        self.ci += 1;
                        self.stack.push(current);

                        if self.current.is_none() {
                            self.ci -= 1;
                            self.stack.pop();

                            // Hack to prevent overflow
                            if self.ci > 0 {
                                self.ci -= 1;
                            }
                            self.current = self.stack.pop();
                        }
                    }
                } else {
                    // the step and criterion do not match
                    match self.stack.last_mut().unwrap().next() {
                        Some(new_cur) => self.current = Some(new_cur),
                        None => {
                            self.ci -= 1;
                            self.current = self.stack.pop();
                        }
                    }
                }
            } else {
                // This must be unreachable, because we look forward for empty criteria in
                //    if self.criteria.len() == self.ci + 1 {
                unreachable!();
            }
        }
        None
    }
}

impl<'a, 'b> Iter<'a, 'b> {
    fn new(root: &'a Value, criteria: &'b Vec<Criterion>) -> Self {
        let root_item = Item::new(root);
        let step = Step::Root;
        let current = Some(StackItem::new(root_item, step));

        Self {
            criteria: criteria,
            stack: vec![],
            current: current,
            ci: 0
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_simple_json() {
        let json = r#"
            {
                "dog": {
                    "name": "Rex"
                }
            }
        "#;

        let root: Value = serde_json::from_str(&json).unwrap();
        let criteria = vec![
            Criterion::Root,
            Criterion::Child("dog".to_owned()),
            Criterion::Child("name".to_owned())
        ];

        let found: Vec<&Value> = Iter::new(&root, &criteria).collect();
        assert_eq!(found, vec!["Rex"]);
    }

    #[test]
    fn test_complex_json() {
        let json = r#"
            {
                "pets": [
                    {
                        "type":"cat",
                        "name":"Tom"
                    },
                    {
                        "type":"dog",
                        "name":"Rex"
                    }
                ],
                "user": {
                    "name":"Sergey",
                    "age":27
                }
            }
        "#;

        let root: Value = serde_json::from_str(&json).unwrap();

        // $.user.age
        let criteria = vec![
            Criterion::Root,
            Criterion::Child("user".to_owned()),
            Criterion::Child("age".to_owned())
        ];
        let found: Vec<&Value> = Iter::new(&root, &criteria).collect();
        assert_eq!(found, vec![27]);

        // $.pets.*.type
        let criteria = vec![
            Criterion::Root,
            Criterion::Child("pets".to_owned()),
            Criterion::AnyChild,
            Criterion::Child("type".to_owned())
        ];
        let found: Vec<&Value> = Iter::new(&root, &criteria).collect();
        assert_eq!(found, vec!["cat", "dog"]);

        // $.pets.*.name
        let criteria = vec![
            Criterion::Root,
            Criterion::Child("pets".to_owned()),
            Criterion::AnyChild,
            Criterion::Child("name".to_owned())
        ];
        let found: Vec<&Value> = Iter::new(&root, &criteria).collect();
        assert_eq!(found, vec!["Tom", "Rex"]);

        // $.user.*
        let criteria = vec![
            Criterion::Root,
            Criterion::Child("user".to_owned()),
            Criterion::AnyChild,
        ];
        let found: Vec<&Value> = Iter::new(&root, &criteria).collect();
        assert_eq!(
            found,
            vec![
                &Value::from(27),
                &Value::from("Sergey")
            ]
        );
    }
}

