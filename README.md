# JSONPath implementation for Rust

The library is in hard development stage.


## Example

```rust
use jsonapth::Selector;
use serde_json;
use serde_json::Value;

let selector = Selector::new("$.inventor.last_name").unwrap();
let jsondoc = r#"
    {
        "inventor": {
            "last_name": "Tesla"
        }
    }
"#;
let json: Value = serde_json::from_str(jsondoc);

assert_eq!(
    selector.find(json),
    Some(&Value::String("Tesla".to_owned()))
)
```
