# JSONPath for Rust

The library is in hard development stage.

## Example

```rust
extern crate jsonpath;
extern crate serde_json;

use jsonpath::Selector;
use serde_json::Value;

fn main() {
    let jsondoc = r#"
        {
             "books": [
                 {
                     "title": "Der schwarze Obelist",
                     "author": "Erich Maria Remarque"
                 },
                 {
                     "title": "Le mur",
                     "author": "Jean-Paul Sartre"
                 }
             ]
        }
    "#;

    // Parse JSON document
    let json: Value = serde_json::from_str(jsondoc).unwrap();

    // Create a JSONPath selector
    let selector = Selector::new("$.books.*.title").unwrap();

    // Apply the selector to the JSON and convert Vec<&Value> into Vec<&str>
    let titles: Vec<&str> = selector.find(&json)
        .map(|t| t.as_str().unwrap())
        .collect();

    assert_eq!(titles, vec!["Der schwarze Obelist", "Le mur"]);
}
```

## Roadmap

* [ ] Operators:
  * [x] `$` - root element
  * [x] `.<name>` - named child element
  * [x] `*` - wildcard (any child item)
  * [x] `[<number>]` - indexed element in array
  * [x] `[<start>:<end>]` - slice
  * [x] `[:<end>]` - slice (to)
  * [x] `[<start>:]` - slice (from)
* [ ] Handy test helpers
* [ ] Good integration test coverage
* [ ] Benchmarks
* [ ] Refactor
* [ ] Improve error messages
* [ ] Review unwraps
* [ ] Review the public API (rename Selector -> Path ?)
* [ ] Publish a new version
* [ ] Mutable iterator
* [ ] Support filters
  * [ ] `[?(<expression>)]` - Filter expression. Expression must evaluate to a boolean value.
  * [ ] `@` - current element

## License

[MIT](https://github.com/greyblake/jsonpath-rs/blob/master/LICENSE) © [Sergey Potapov](http://greyblake.com)

## Contributors

- [greyblake](https://github.com/greyblake) Sergey Potapov - creator, maintainer.
