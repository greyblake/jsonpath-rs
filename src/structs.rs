// TODO:
// - add IndexedChild(usize)
#[derive(Debug, Clone, PartialEq)]
pub enum Criterion {
    // $
    Root,

    // .name
    NamedChild(String),

    // .*
    AnyChild,

    // [123]
    IndexedChild(usize),
}

// A step during traversing JSON tree
#[derive(Debug, Clone, PartialEq)]
pub enum Step<'a> {
    Root,
    Key(&'a str),
    Index(usize)
}


// TODO: write unit tests
pub fn matches(step: &Step, criterion: &Criterion) -> bool {
    match criterion {
        &Criterion::Root => {
            match step {
                &Step::Root => true,
                _ => false
            }
        },
        &Criterion::NamedChild(ref child_name) => {
            match step {
                &Step::Key(ref key) => child_name == key,
                _ => false
            }
        },
        &Criterion::AnyChild => {
            match step {
                &Step::Key(_) => true,
                &Step::Index(_) => true,
                _ => false
            }
        }
        &Criterion::IndexedChild(index) => {
            match step {
                &Step::Index(idx) => index == idx,
                _ => false
            }
        }
    }
}


use serde_json::Value;
use std::slice::Iter;
use std::iter::Enumerate;


pub enum ItemIter<'a> {
    Array(Enumerate<Iter<'a, Value>>),
    Object(::serde_json::map::Iter<'a>)
}

pub struct Item<'a> {
    pub value: &'a Value,
    pub iter: Option<ItemIter<'a>>
}

impl<'a> Item<'a> {
    pub fn new(value: &'a Value) -> Self {
        let iter = match value {
            &Value::Array(ref vec) => Some(ItemIter::Array(vec.iter().enumerate())),
            &Value::Object(ref map) => Some(ItemIter::Object(map.iter())),
            _ => None
        };
        Self { value, iter }
    }

    pub fn next(&mut self) -> Option<(Self, Step<'a>)> {
        match self.iter {
            Some(ref mut item_iter) => {
                match item_iter {
                    &mut ItemIter::Array(ref mut iter) => {
                        match iter.next() {
                            Some((index, val)) => {
                                let sub_item = Item::new(val);
                                let step = Step::Index(index);
                                Some((sub_item, step))
                            },
                            None => None
                        }
                    },
                    &mut ItemIter::Object(ref mut iter) => {
                        match iter.next() {
                            Some((key, val)) => {
                                let sub_item = Item::new(val);
                                let step = Step::Key(key);
                                Some((sub_item, step))
                            }
                            None => None
                        }
                    }
                }
            },
            None => None
        }
    }
}

pub struct StackItem<'a> {
    pub item: Item<'a>,
    pub step: Step<'a>
}

impl<'a> StackItem<'a> {
    pub fn new(item: Item<'a>, step: Step<'a>) -> Self {
        Self { item, step }
    }

    pub fn next(&mut self) -> Option<Self> {
        self.item.next().map( |(sub_item, step)| Self::new(sub_item, step) )
    }
}

