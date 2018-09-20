extern crate json;

use json::JsonValue;
use std::io::Read;

fn main() {
    match json::parse(get_input().as_str()) {
        Ok(val) => println!("{}", unquote(val).to_string()),
        Err(_) => println!("That's not even JSON!")
    }
}

/// get the first os arg if there is one, otherwise consume all of stdin
fn get_input<'a>() -> String {
    let mut os_args: Vec<String> = std::env::args().collect();
    if os_args.len() > 1 {
        return os_args.remove(1);
    } else {
        let mut buffer = std::string::String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        return buffer;
    }
}

/// recursively implemented unquoting function
fn unquote(json_value: JsonValue) -> JsonValue {
    match json_value {
        // if it's an object, recurse on the object values
        JsonValue::Object(json_object) => {
            let mut new_object = json::JsonValue::new_object();
            for (key, val) in json_object.iter() {
                new_object[key] = unquote(val.clone())
            }
            new_object
        }
        // if it's an array, recurse on the array entries
        JsonValue::Array(json_entries) => {
            JsonValue::Array(json_entries.into_iter().map(unquote).collect())
        }
        // if it's a string, attempt to parse the string, see if it's valid json, and if it is, recurse on that
        JsonValue::String(json_string) =>
            match json::parse(json_string.as_str()) {
                Ok(unquoted) => unquote(unquoted),
                Err(_) => JsonValue::String(json_string),
            }
        _ => json_value
    }
}