extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

macro_rules! assert_jsonpath_f64 {
    ($path:expr, $expected:expr) => {
        let mut data_struct = String::new();
        let mut data_file = File::open("tests/data.json").unwrap();
        let _ = data_file.read_to_string(&mut data_struct).unwrap();
        assert_jsonpath!(data_struct.as_str(), $path, f64, as_f64, $expected);
    };
}

macro_rules! assert_jsonpath_str {
    ($path:expr, $expected:expr) => {
        let mut data_struct = String::new();
        let mut data_file = File::open("tests/data.json").unwrap();
        let _ = data_file.read_to_string(&mut data_struct).unwrap();
        assert_jsonpath!(data_struct.as_str(), $path, &str, as_str, $expected);
    };
}

macro_rules! assert_jsonpath {
    ($json:expr, $path:expr, $type:ty, $convert:ident, $expected:expr) => {
        let value: Value = serde_json::from_str($json).unwrap();
        let selector = Selector::new($path).unwrap();
        let selected_values: Vec<$type> = selector
            .find(&value)
            .map(|x| {
                // println!("{:?}", x);
                x.$convert()
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
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
    assert_jsonpath_f64!("$.store.books[1:2].price", [12.99, 9.0]);
}

#[test]
fn test_slice_to() {
    assert_jsonpath_f64!("$.store.books[:3].price", [8.95, 12.99, 9.0]);
}

#[test]
fn test_slice_from() {
    assert_jsonpath_f64!("$.store.books[1:].price", [12.99, 9.0, 22.99]);
}

#[test]
fn test_filter() {
    assert_jsonpath_f64!("$.store.books[?(@.category == 'reference')].price", [8.95]);
    assert_jsonpath_str!(
        "$.store.books[?(@.category == 'fiction')].title",
        ["Sword of Honour", "Moby Dick", "The Lord of the Rings"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price == 8.95)].title",
        ["Sayings of the Century"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price != 8.95)].title",
        ["Sword of Honour", "Moby Dick", "The Lord of the Rings"]
    );
    assert_jsonpath_str!("$.store.books[?(@.price == 9)].title", ["Moby Dick"]);
    assert_jsonpath_str!(
        "$.store.books[?(@.price != 9)].title",
        [
            "Sayings of the Century",
            "Sword of Honour",
            "The Lord of the Rings"
        ]
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
fn test_filter_lower_greater_comparison() {
    assert_jsonpath_str!(
        "$.store.books[?(@.title > 'Sc')].author",
        ["Evelyn Waugh", "J. R. R. Tolkien"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.title < 'Sc')].author",
        ["Nigel Rees", "Herman Melville"]
    );
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
fn test_filter_lower_greater_or_equal_comparison() {
    assert_jsonpath_str!(
        "$.store.books[?(@.title >= 'Sc')].author",
        ["Evelyn Waugh", "J. R. R. Tolkien"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.title <= 'Sc')].author",
        ["Nigel Rees", "Herman Melville"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price >= 9.99)].title",
        ["Sword of Honour", "The Lord of the Rings"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price <= 9.99)].title",
        ["Sayings of the Century", "Moby Dick"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price >= 10)].title",
        ["Sword of Honour", "The Lord of the Rings"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.price <= 10)].title",
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
fn test_filter_and_or() {
    assert_jsonpath_str!(
        "$.store.books[?(@.category == 'fiction') && ?(@.price < 10)].title",
        ["Moby Dick"]
    );

    assert_jsonpath_str!(
        "$.store.books[?(@.category == 'reference') || ?(@.price == 9)].title",
        ["Sayings of the Century", "Moby Dick"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.category == 'reference') || ?(@.price == 9) || ?(@.author == 'Evelyn Waugh')].title",
        [
            "Sayings of the Century",
            "Sword of Honour",
            "Moby Dick"
        ]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.category == 'reference') || ?(@.category == 'fiction') && ?(@.price == 9)].title",
        [
            "Sayings of the Century",
            "Moby Dick",
        ]
    );
}

#[test]
fn test_filter_with_expression_on_right_side() {
    assert_jsonpath_str!(
        "$.store.books[?(@.price > $.store.books[?(@.title == 'Moby Dick')].price)].title",
        ["Sword of Honour", "The Lord of the Rings",]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author > $.store.books[?(@.title == 'Moby Dick')].author)].title",
        ["Sayings of the Century", "The Lord of the Rings"]
    );

    assert_jsonpath_str!(
        "$.store.books[?(@.price >= $.store.books[?(@.title == 'Moby Dick')].price)].title",
        ["Sword of Honour", "Moby Dick", "The Lord of the Rings",]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author >= $.store.books[?(@.title == 'Moby Dick')].author)].title",
        [
            "Sayings of the Century",
            "Moby Dick",
            "The Lord of the Rings"
        ]
    );

    assert_jsonpath_str!(
        "$.store.books[?(@.price < $.store.books[?(@.title == 'Moby Dick')].price)].title",
        ["Sayings of the Century"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author < $.store.books[?(@.title == 'Moby Dick')].author)].title",
        ["Sword of Honour"]
    );

    assert_jsonpath_str!(
        "$.store.books[?(@.price <= $.store.books[?(@.title == 'Moby Dick')].price)].title",
        ["Sayings of the Century", "Moby Dick"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author <= $.store.books[?(@.title == 'Moby Dick')].author)].title",
        ["Sword of Honour", "Moby Dick"]
    );

    assert_jsonpath_str!(
        "$.store.books[?(@.price == $.store.books[?(@.title == 'Moby Dick')].price)].title",
        ["Moby Dick"]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author == $.store.books[?(@.title == 'Moby Dick')].author)].title",
        ["Moby Dick"]
    );

    assert_jsonpath_str!(
        "$.store.books[?(@.price != $.store.books[?(@.title == 'Moby Dick')].price)].title",
        [
            "Sayings of the Century",
            "Sword of Honour",
            "The Lord of the Rings"
        ]
    );
    assert_jsonpath_str!(
        "$.store.books[?(@.author != $.store.books[?(@.title == 'Moby Dick')].author)].title",
        [
            "Sayings of the Century",
            "Sword of Honour",
            "The Lord of the Rings"
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
