use stack::Stack;

enum Criterion {
    Root,
    Child(String)
}


use serde_json::Value;

struct Iter<'a, 'b> {
    root: &'a Value,
    criteria: &'b Vec<Criterion>,
    ci: usize,
    value_stack: Vec<&'a Value>
}

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.find_next()
    }
}

impl<'a, 'b> Iter<'a, 'b> {
    fn new(root: &'a Value, criteria: &'b Vec<Criterion>) -> Self {
        let mut iter = Self {
            root: root,
            criteria: criteria,
            value_stack: Vec::new(),
            ci: 0
        };
        iter.init();
        iter
    }

    fn init(&mut self) {
        self.value_stack.push(self.root);
        self.ci = 1;

        while let Some(criterion) = self.criteria.get(self.ci) {
            let last_val = self.value_stack.last();

            match criterion {
                &Criterion::Root => unreachable!(),
                &Criterion::Child(ref child_name) => {
                    match last_val {
                        Some(val) => {
                            match val {
                                &Value::Object(obj) => {
                                    match obj.get(child_name) {
                                        Some(v) => {
                                            self.value_stack.push(v);
                                            self.ci += 1;
                                        },
                                        None => panic!("Another none")
                                    }
                                }
                                _ => panic!("ELSE")
                            }
                        }
                        None => panic!("None!!!!")
                    }
                }
            }
        }
    }

    fn find_next(&mut self) -> Option<&'a Value> {
        if self.criteria.get(self.ci).is_none() {
            self.ci -= 1;
            return self.value_stack.pop();
        } else {
            if self.value_stack.is_empty() {
                None
            } else {
                panic!("BANG");
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_iter() {
        let json = r#"
            {
                "dog": {
                    "name": "Rex"
                }
            }
        "#;

        let root: Value = serde_json::from_str(json).unwrap();

        let criteria = vec![Criterion::Root];
        let mut iter = Iter::new(&root, &criteria);
        assert_eq!(iter.next(), Some(&root));
        assert_eq!(iter.next(), None);


        let criteria = vec![
            Criterion::Root,
            Criterion::Child("dog".to_owned()),
            Criterion::Child("name".to_owned())
        ];
        let mut iter = Iter::new(&root, &criteria);
        assert_eq!(iter.next(), Some(&Value::String("Rex".to_owned())));
        assert_eq!(iter.next(), None);
    }
}

