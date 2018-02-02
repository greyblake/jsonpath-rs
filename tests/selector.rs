extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

const JSONDOC: &'static str = r#"
    { "store": {
        "books": [
          { "category": "reference",
            "author": "Nigel Rees",
            "title": "Sayings of the Century",
            "price": 8.95
          },
          { "category": "fiction",
            "author": "Evelyn Waugh",
            "title": "Sword of Honour",
            "price": 12.99
          },
          { "category": "fiction",
            "author": "Herman Melville",
            "title": "Moby Dick",
            "isbn": "0-553-21311-3",
            "price": 8.99
          },
          { "category": "fiction",
            "author": "J. R. R. Tolkien",
            "title": "The Lord of the Rings",
            "isbn": "0-395-19395-8",
            "price": 22.99
          }
        ],
        "bicycle": {
          "color": "red",
          "price": 19.95
        }
      }
    }
"#;

#[test]
fn test_find() {
    let json: Value = serde_json::from_str(JSONDOC).unwrap();
    let selector = Selector::new("$.store.bicycle.price").unwrap();

    let bicycle_price = selector.find(&json).nth(0).unwrap();
    assert_eq!(bicycle_price, 19.95);
}

#[test]
fn test_index() {
    let json: Value = serde_json::from_str(JSONDOC).unwrap();
    let selector = Selector::new("$.store.books[2].title").unwrap();

    let title = selector.find(&json).nth(0).unwrap();
    assert_eq!(title, "Moby Dick");
}

#[test]
fn test_slice() {
    let json: Value = serde_json::from_str(JSONDOC).unwrap();
    let selector = Selector::new("$.store.books[1:2].price").unwrap();

    let prices: Vec<f64> = selector.find(&json).map(|x| x.as_f64().unwrap()).collect();
    assert_eq!(prices, vec![12.99, 8.99]);
}

#[test]
fn test_slice_to() {
    let json: Value = serde_json::from_str(JSONDOC).unwrap();
    let selector = Selector::new("$.store.books[:3].price").unwrap();

    let prices: Vec<f64> = selector.find(&json).map(|x| x.as_f64().unwrap()).collect();
    assert_eq!(prices, vec![8.95, 12.99, 8.99]);
}
