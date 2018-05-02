use filter;
use serde_json::Value;
use std::slice::Iter;
use std::iter::Enumerate;

#[derive(Debug, Clone, PartialEq)]
pub enum Criterion {
    // $
    Root,
    // @
    Element,
    // .name
    NamedChild(String),
    // ?(path)
    Filter(Vec<Criterion>),
    // path
    SubExpression(Vec<Criterion>),
    // .*
    AnyChild,
    // [123]
    IndexedChild(usize),
    // [10:20]
    Slice(::std::ops::Range<usize>),
    // [:7]
    SliceTo(::std::ops::RangeTo<usize>),
    // [4:]
    SliceFrom(usize),
    // [values]
    Array(Vec<Criterion>),
    // ==
    Equal,
    // !=
    Different,
    // >
    Greater,
    // >=
    GreaterOrEqual,
    // <
    Lower,
    // <=
    LowerOrEqual,
    // 'content'
    Literal(String),
    // 10
    Number(i64),
    // 9.99
    Float(f64),
    // &&
    And,
    // ||
    Or,
}

// A step during traversing JSON tree
#[derive(Debug, Clone, PartialEq)]
pub enum Step<'a> {
    Root,
    Key(&'a str),
    Index(usize),
}

// TODO: write unit tests
pub fn matches<'a>(stack: &mut StackItem, criterion: &Criterion, root: &StackItem<'a>) -> bool {
    let step = stack.step.clone();
    match *criterion {
        Criterion::Root => match step {
            Step::Root => true,
            _ => false,
        },
        Criterion::Element => false,
        Criterion::Equal => false,
        Criterion::Different => false,
        Criterion::Greater => false,
        Criterion::GreaterOrEqual => false,
        Criterion::Lower => false,
        Criterion::LowerOrEqual => false,
        Criterion::And => false,
        Criterion::Or => false,
        Criterion::Literal(ref _content) => false,
        Criterion::Number(ref _value) => false,
        Criterion::Float(ref _value) => false,
        Criterion::Array(ref _value) => false,
        Criterion::SubExpression(ref _expr) => false,
        Criterion::NamedChild(ref child_name) => match step {
            Step::Key(key) => child_name == key,
            _ => false,
        },
        Criterion::Filter(ref path) => {
            let mut filter_stack = stack.clone();
            filter::process_filter(&mut filter_stack, path, root)
        }
        Criterion::AnyChild => match step {
            Step::Key(_) => true,
            Step::Index(_) => true,
            _ => false,
        },
        Criterion::IndexedChild(index) => match step {
            Step::Index(idx) => index == idx,
            _ => false,
        },
        Criterion::Slice(ref range) => match step {
            Step::Index(idx) => range.start <= idx && idx <= range.end,
            _ => false,
        },
        Criterion::SliceTo(ref range_to) => match step {
            Step::Index(idx) => idx < range_to.end,
            _ => false,
        },
        Criterion::SliceFrom(from) => match step {
            Step::Index(idx) => from <= idx,
            _ => false,
        },
    }
}

pub enum ItemIter<'a> {
    Array(Enumerate<Iter<'a, Value>>),
    Object(::serde_json::map::Iter<'a>),
}

pub struct Item<'a> {
    pub value: &'a Value,
    pub iter: Option<ItemIter<'a>>,
}

impl<'a> Item<'a> {
    pub fn new(value: &'a Value) -> Self {
        let iter = match *value {
            Value::Array(ref vec) => Some(ItemIter::Array(vec.iter().enumerate())),
            Value::Object(ref map) => Some(ItemIter::Object(map.iter())),
            _ => None,
        };
        Self { value, iter }
    }

    pub fn next(&mut self) -> Option<(Self, Step<'a>)> {
        match self.iter {
            Some(ref mut item_iter) => match *item_iter {
                ItemIter::Array(ref mut iter) => match iter.next() {
                    Some((index, val)) => {
                        let sub_item = Item::new(val);
                        let step = Step::Index(index);
                        Some((sub_item, step))
                    }
                    None => None,
                },
                ItemIter::Object(ref mut iter) => match iter.next() {
                    Some((key, val)) => {
                        let sub_item = Item::new(val);
                        let step = Step::Key(key);
                        Some((sub_item, step))
                    }
                    None => None,
                },
            },
            None => None,
        }
    }
}

impl<'a> Clone for Item<'a> {
    fn clone(&self) -> Item<'a> {
        Item::new(self.value)
    }
}

#[derive(Clone)]
pub struct StackItem<'a> {
    pub item: Item<'a>,
    pub step: Step<'a>,
}

impl<'a> StackItem<'a> {
    pub fn new(item: Item<'a>, step: Step<'a>) -> Self {
        Self { item, step }
    }

    pub fn next(&mut self) -> Option<Self> {
        self.item
            .next()
            .map(|(sub_item, step)| Self::new(sub_item, step))
    }
}
