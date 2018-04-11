extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

macro_rules! assert_jsonpath_f64 {
    ($path: expr, $expected: expr) => {
        let mut data_struct = String::new();
        let mut data_file = File::open("tests/data.json").unwrap();
        let _ = data_file.read_to_string(&mut data_struct).unwrap();
        assert_jsonpath!(data_struct.as_str(), $path, f64, as_f64, $expected);
    };
}

macro_rules! assert_jsonpath_str {
    ($path: expr, $expected: expr) => {
        let mut data_struct = String::new();
        let mut data_file = File::open("tests/data.json").unwrap();
        let _ = data_file.read_to_string(&mut data_struct).unwrap();
        assert_jsonpath!(data_struct.as_str(), $path, &str, as_str, $expected);
    };
}

macro_rules! assert_jsonpath {
    ($json: expr, $path: expr, $type: ty, $convert: ident, $expected: expr) => {
        let value: Value = serde_json::from_str($json).unwrap();
        let selector = Selector::new($path).unwrap();
        let selected_values: Vec<$type> = selector
            .find(&value)
            .map(|x| x.$convert().unwrap())
            .collect();
        assert_eq!(selected_values, $expected);
    };
}

#[test]
fn test_find() {
    assert_jsonpath_f64!("$.store.bicycle.price", [19.95]);
}

#[test]
fn test_index() {
    assert_jsonpath_str!("$.store.books[2].title", ["Moby Dick"]);
}

#[test]
fn test_slice() {
    assert_jsonpath_f64!("$.store.books[1:2].price", [12.99, 8.99]);
}

#[test]
fn test_slice_to() {
    assert_jsonpath_f64!("$.store.books[:3].price", [8.95, 12.99, 8.99]);
}

#[test]
fn test_slice_from() {
    assert_jsonpath_f64!("$.store.books[1:].price", [12.99, 8.99, 22.99]);
}

#[test]
fn test_filter() {
    assert_jsonpath_f64!("$.store.books[?(@.category == 'reference')].price", [8.95]);
    assert_jsonpath_str!(
        "$.store.books[?(@.category == 'fiction')].title",
        ["Sword of Honour", "Moby Dick", "The Lord of the Rings"]
    );
}

#[test]
fn test_filter_array_string_conditions() {
    assert_jsonpath_f64!(
        "$.store.books[?(@.author == ['Evelyn Waugh' ])].price",
        [12.99]
    );
    assert_jsonpath_f64!(
        "$.store.books[?(@.author == ['Evelyn Waugh', 'Nigel Rees'])].price",
        [8.95, 12.99]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author == ['Evelyn Waugh'])].title",
        ["Sword of Honour"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author == ['Evelyn Waugh', 'Nigel Rees'])].title",
        ["Sayings of the Century", "Sword of Honour"]
    );
}

#[test]
fn test_filter_number_comparison() {
    assert_jsonpath_str!(
        "$.store.books[?(@.price > 9.99)].title",
        ["Sword of Honour", "The Lord of the Rings"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price < 9.99)].title",
        ["Sayings of the Century", "Moby Dick"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price > 10)].title",
        ["Sword of Honour", "The Lord of the Rings"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price < 10)].title",
        ["Sayings of the Century", "Moby Dick"]
    );
}

#[test]
fn test_filter_with_absolute_condition() {
    assert_jsonpath_str!(
        "$.store.books[?($.store.books[2].category == 'fiction')].title",
        [
            "Sayings of the Century",
            "Sword of Honour",
            "Moby Dick",
            "The Lord of the Rings",
        ]
    );
    assert_jsonpath_str!(
        "$.store.books[?($.store.books[2].category != 'reference')].title",
        [
            "Sayings of the Century",
            "Sword of Honour",
            "Moby Dick",
            "The Lord of the Rings",
        ]
    );
}

#[test]
fn test_root() {
    let json = r#"
    {"a": 10}
  "#;
    let value: Value = serde_json::from_str(json).unwrap();
    let selector = Selector::new("$").unwrap();
    let _found_values: Vec<&Value> = selector.find(&value).collect();
}
